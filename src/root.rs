use gpui::*;
use gpui::{Action, Model};
use serde::Deserialize;

use crate::button::*;
use crate::consts::*;
use crate::list::*;
use crate::registration::*;
use crate::styles::*;
use crate::text_input::*;

pub struct Root {
    pub registration: Model<Registration>,
    focus_handle: FocusHandle,
}

// 1. Define the struct (outside the macro)
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TypeChar {
    pub text: String,
}

// Manually implement the Action trait (replaces the macro)
impl Action for TypeChar {
    fn name(&self) -> &'static str {
        "root::TypeChar"
    }

    fn boxed_clone(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn partial_eq(&self, action: &dyn Action) -> bool {
        action
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| a == self)
    }

    fn debug_name() -> &'static str {
        "TypeChar"
    }

    fn build(value: private::serde_json::Value) -> Result<Box<dyn Action>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Root {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        // Create the model in the app context
        let registration = cx.new_model(|_| Registration::new());
        let focus_handle = cx.focus_handle();

        // Focus once during initialization
        cx.focus(&focus_handle);

        Self {
            registration,
            focus_handle,
        }
    }

    fn render_tab_bar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let reg = self.registration.read(cx);
        let active = reg.active_tab;

        div()
            .flex()
            .bg(rgb(LIST_COLOR))
            .p_2()
            .gap_4()
            .child(self.tab_button(
                ButtonType::Settings,
                ActiveTab::Settings,
                active == ActiveTab::Settings,
            ))
            .child(self.tab_button(
                ButtonType::Manage,
                ActiveTab::Manage,
                active == ActiveTab::Manage,
            ))
            .child(self.tab_button(
                ButtonType::Employees,
                ActiveTab::Employees,
                active == ActiveTab::Employees,
            ))
    }

    fn tab_button(&self, b_type: ButtonType, tab: ActiveTab, is_active: bool) -> impl IntoElement {
        let reg_model = self.registration.clone();
        let variant = if is_active {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Secondary
        };

        Button::new(b_type, variant).on_click(move |_, cx| {
            cx.stop_propagation();
            reg_model.update(cx, |r, cx| {
                if r.active_tab != tab {
                    r.active_tab = tab;
                    r.focused_field = FocusField::None;
                    cx.notify();
                }
            });
        })
    }

    fn render_settings_tab(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        let reg = self.registration.read(cx);
        let reg_model = self.registration.clone();

        // Local clones for callbacks
        let reg_name = reg_model.clone();
        let reg_pass = reg_model.clone();
        let reg_conn = reg_model.clone();

        let status_color = if reg.db_connected {
            STATUS_COLOR_GREEN
        } else {
            STATUS_COLOR_RED
        };
        let status_text = if reg.db_connected {
            "Connected"
        } else {
            "Disconnected"
        };

        div()
            .flex_col()
            .gap_4()
            .w(px(600.0))
            .child(
                div()
                    .child(format!("STATUS: {}", status_text))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(status_color)),
            )
            .child(
                TextInput::new("Database Name", reg.db_name.clone())
                    .focused(reg.focused_field == FocusField::DbName)
                    .on_click(move |_, cx| {
                        reg_name.update(cx, |r, _| r.focused_field = FocusField::DbName);
                    }),
            )
            .child(
                TextInput::new("Password", reg.db_password.clone())
                    .password(true)
                    .focused(reg.focused_field == FocusField::DbPassword)
                    .on_click(move |_, cx| {
                        reg_pass.update(cx, |r, _| r.focused_field = FocusField::DbPassword);
                    }),
            )
            .child(
                Button::new(ButtonType::Connect, ButtonVariant::Primary).on_click(move |_, cx| {
                    cx.stop_propagation();
                    reg_conn.update(cx, |reg_state, model_cx| {
                        reg_state.connect_to_db(
                            reg_state.db_name.clone(),
                            reg_state.db_password.clone(),
                            model_cx,
                        );
                    });
                }),
            )
            .into_any_element()
    }

    fn render_input_field(
        &self,
        label: &'static str,
        value: String,
        is_focused: bool,
        on_click: impl Fn(&mut WindowContext) + 'static,
    ) -> impl IntoElement {
        div()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(WHITE_COLOR))
                    .child(label),
            )
            .child(
                div()
                    .p_2()
                    .bg(rgb(LIST_COLOR))
                    .text_color(rgb(WHITE_COLOR))
                    .border_2()
                    .hover(|this| this.bg(rgb(BUTTON_COLOR_HOVER)))
                    .cursor_text()
                    // Highlight border if focused
                    .border_color(if is_focused {
                        rgb(PRIMARY_COLOR)
                    } else {
                        rgb(BUTTON_COLOR)
                    })
                    .child(if value.is_empty() {
                        " ".to_string()
                    } else {
                        value
                    })
                    .on_mouse_down(MouseButton::Left, move |_, cx| on_click(cx)),
            )
    }

    fn render_manage_tab(&self, _cx: &mut ViewContext<Self>) -> AnyElement {
        div()
            .flex_row()
            .gap_8()
            .child(
                div()
                    .flex_col()
                    .gap_2()
                    .w(px(200.0))
                    .child(Button::new(ButtonType::Insert, ButtonVariant::Neutral))
                    .child(Button::new(ButtonType::Edit, ButtonVariant::Neutral))
                    .child(Button::new(ButtonType::Delete, ButtonVariant::Neutral)),
            )
            .child(
                div().flex_col().child("Headshot Preview:").child(
                    div()
                        .size(px(320.0))
                        .bg(rgb(DARK_MODE_COLOR))
                        .border_1()
                        .border_color(rgb(WHITE_COLOR))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child("Webcam Feed Placeholder"),
                ),
            )
            .into_any_element()
    }

    fn crud_button(&self, label: &'static str) -> impl IntoElement {
        div()
            .p_2()
            .bg(rgb(BUTTON_COLOR))
            .child(label)
            .cursor_pointer()
    }

    fn render_employees_tab(&self, _cx: &mut ViewContext<Self>) -> AnyElement {
        //div().child("Empty List").into_any_element()
        div()
            .size_full()
            .flex_col()
            .bg(rgb(LIST_COLOR))
            .child("List")
            .into_any_element()
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_tab = self.registration.read(cx).active_tab;
        let reg_model = self.registration.clone(); // Clone for the closure

        div()
            .id("root-shell") // Giving the root an ID helps with focus routing
            .track_focus(&self.focus_handle)
            .key_context("RootView") // Add this!
            .on_key_down(move |ev, cx| {
                let keystroke = &ev.keystroke.key;
                // Debug print to see if keys are actually arriving
                // println!("Key pressed: {}", keystroke);

                reg_model.update(cx, |reg, model_cx| {
                    reg.handle_text_input(keystroke);
                    model_cx.notify();
                });
            })
            .flex_col()
            .size_full()
            .bg(rgb(DARK_MODE_COLOR))
            .child(self.render_tab_bar(cx))
            .child(div().flex_grow().p_4().child(match active_tab {
                ActiveTab::Settings => self.render_settings_tab(cx),
                ActiveTab::Manage => self.render_manage_tab(cx),
                ActiveTab::Employees => div().child("Employee List").into_any_element(),
            }))
    }
}
