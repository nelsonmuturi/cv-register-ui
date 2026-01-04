use gpui::*;
use gpui::{Action, Model};
use serde::Deserialize;

use crate::button::*;
use crate::consts::*;
use crate::list::*;
use crate::registration::*;
use crate::styles::*;

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
                "Settings",
                ActiveTab::Settings,
                active == ActiveTab::Settings,
            ))
            .child(self.tab_button("Manage", ActiveTab::Manage, active == ActiveTab::Manage))
            .child(self.tab_button("List", ActiveTab::Employees, active == ActiveTab::Employees))
    }

    fn tab_button(&self, label: &'static str, tab: ActiveTab, is_active: bool) -> impl IntoElement {
        let reg_model = self.registration.clone();
        div()
            .px_4()
            .py_2()
            .rounded_md()
            .bg(if is_active {
                rgb(PRIMARY_COLOR)
            } else {
                rgb(BUTTON_COLOR)
            })
            .child(label)
            .cursor_pointer()
            .font_weight(FontWeight::BOLD)
            .on_mouse_down(MouseButton::Left, move |_, cx| {
                cx.stop_propagation(); // Prevent overlap issues
                reg_model.update(cx, |r, cx| {
                    if r.active_tab != tab {
                        r.active_tab = tab;
                        r.focused_field = FocusField::None;
                        cx.notify(); // Notify model
                    }
                });
            })
    }

    fn render_settings_tab(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        //div().child("Empty Settings").into_any_element()

        let reg = self.registration.read(cx);

        // Clone for the first input field's closure
        let reg_for_name = self.registration.clone();
        // Clone for the second input field's closure
        let reg_for_pass = self.registration.clone();
        // Clone the handle to move into db-connection mouse-down closure
        let reg_model = self.registration.clone();

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
            .flex()
            .flex_col()
            .gap_y(px(16.0))
            .w(px(600.0))
            .gap_4()
            .child(
                div()
                    .child(format!("STATUS: {}", status_text))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(status_color)),
            )
            .gap_4()
            .child(self.render_input_field(
                "Database Name",
                reg.db_name.clone(), // Pass owned String
                reg.focused_field == FocusField::DbName,
                move |cx| {
                    reg_for_name.update(cx, |r, _| r.focused_field = FocusField::DbName);
                },
            ))
            .child(self.render_input_field(
                "Password",
                "*".repeat(reg.db_password.len()), // Already an owned String
                reg.focused_field == FocusField::DbPassword,
                move |cx| {
                    reg_for_pass.update(cx, |r, _| r.focused_field = FocusField::DbPassword);
                },
            ))
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .font_weight(FontWeight::BOLD)
                    .bg(rgb(PRIMARY_COLOR))
                    .p_2()
                    .rounded_md()
                    .child("Connect to Postgres DB")
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, move |_, cx| {
                        // Call connect_to_db logic
                        cx.stop_propagation(); // Prevent overlap issues

                        // Use .update to bridge the context and access the latest state
                        reg_model.update(cx, |reg_state, model_cx| {
                            // reg_state is &mut Registration (the latest model data)
                            // model_cx is &mut ModelContext<Registration>
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
        // div().child("Empty Manage").into_any_element()
        div()
            .flex_row()
            .gap_8()
            .child(
                // Left Side: CRUD Controls
                div()
                    .flex_col()
                    .gap_2()
                    .w(px(200.0))
                    .child(self.crud_button("Insert"))
                    .child(self.crud_button("Edit"))
                    .child(self.crud_button("Delete")),
            )
            .child(
                // Right Side: Mugshot Viewport
                div().flex_col().child("Headshot Preview:").child(
                    div()
                        .size(px(320.0))
                        .bg(rgb(0x000000))
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
