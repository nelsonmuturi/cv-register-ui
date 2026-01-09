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
                ButtonType::Persons,
                ActiveTab::Persons,
                active == ActiveTab::Persons,
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

                    // Focus name input automatically if switching to Persons
                    r.focused_field = if tab == ActiveTab::Persons {
                        FocusField::PersonName
                    } else {
                        FocusField::None
                    };

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

    fn render_persons_tab(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        let reg = self.registration.read(cx);
        let reg_model = self.registration.clone();

        div()
            .flex()
            .size_full()
            .flex_row()
            .gap_6()
            .child(
                // LEFT: Scrollable list
                div()
                    .flex_none()
                    .w_1_3()
                    .flex_col()
                    .bg(rgb(LIST_COLOR))
                    .rounded_md()
                    .p_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(WHITE_COLOR))
                            .mb_2()
                            .child("Registered Persons"),
                    )
                    .child(
                        div()
                            .flex_col()
                            .gap_1()
                            .children(reg.persons.iter().map(|p| {
                                let p_clone = p.clone();
                                let is_selected = reg
                                    .selected_person
                                    .as_ref()
                                    .is_some_and(|s| s.person_id == p.person_id);
                                let reg_select = reg_model.clone();

                                div()
                                    .p_2()
                                    .rounded_sm()
                                    .bg(if is_selected {
                                        rgb(PRIMARY_COLOR)
                                    } else {
                                        rgb(BUTTON_COLOR)
                                    })
                                    .hover(|s| s.bg(rgb(BUTTON_COLOR_HOVER)))
                                    .child(p.full_name.clone())
                                    .on_mouse_down(MouseButton::Left, move |_, cx| {
                                        reg_select.update(cx, |r, cx| {
                                            r.select_person(p_clone.clone());
                                            cx.notify();
                                        });
                                    })
                            })),
                    ),
            )
            .child(
                // RIGHT: Form
                div()
                    .flex_col()
                    .w_2_3()
                    .gap_4()
                    .child(
                        TextInput::new("Full Name", reg.person_draft.full_name.clone())
                            .focused(reg.focused_field == FocusField::PersonName)
                            .on_click({
                                let reg_model = reg_model.clone();
                                move |_, cx| {
                                    reg_model.update(cx, |r, _| {
                                        r.focused_field = FocusField::PersonName
                                    });
                                }
                            }),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(WHITE_COLOR))
                            .child("Person Type")
                            .child(self.render_person_type_selector(cx)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(WHITE_COLOR))
                            .child("Access Level")
                            .child(self.render_access_level_selector(cx)),
                    )
                    .child(
                        div()
                            .flex_row()
                            .gap_4()
                            .child(
                                Button::new(ButtonType::Insert, ButtonVariant::Primary).on_click({
                                    let reg_model = reg_model.clone();
                                    move |_, cx| {
                                        reg_model.update(cx, |r, cx| r.save_person(cx));
                                    }
                                }),
                            )
                            .child(
                                Button::new(ButtonType::Delete, ButtonVariant::Secondary).on_click(
                                    {
                                        let reg_model = reg_model.clone();
                                        move |_, cx| {
                                            reg_model.update(cx, |r, cx| r.delete_person(cx));
                                        }
                                    },
                                ),
                            ),
                    )
                    .child(
                        div()
                            .w_1_2()
                            .mt_auto()
                            .h(px(300.0))
                            .bg(rgb(DARK_MODE_COLOR))
                            .border_1()
                            .border_color(rgb(WHITE_COLOR))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child("Webcam Preview Placeholder"),
                    ),
            )
            .into_any_element()
    }

    // gem-2026-01-07: Toggle-based selector for Person Type
    fn render_person_type_selector(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        let reg = self.registration.read(cx);
        let current_type = &reg.person_draft.person_type;
        let reg_model = self.registration.clone();

        div()
            .flex_row()
            .gap_2()
            .children(
                ["employee", "customer", "visitor", "other"]
                    .iter()
                    .map(|t| {
                        let t_str = t.to_string();
                        let is_active = current_type == &t_str;

                        div()
                            .px_2()
                            .py_1()
                            .bg(if is_active {
                                rgb(0x4a90e2)
                            } else {
                                rgb(0x333333)
                            })
                            .child(t_str.clone())
                            .on_mouse_down(MouseButton::Left, {
                                let reg_model = reg_model.clone();
                                let t_str = t_str.clone();
                                move |_, cx| {
                                    reg_model.update(cx, |r, cx| {
                                        r.update_person_type(t_str.clone());
                                        cx.notify();
                                    });
                                }
                            })
                    }),
            )
            .into_any_element()
    }

    // gem-2026-01-07: Toggle-based selector for Access Level
    fn render_access_level_selector(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        let reg = self.registration.read(cx);
        let current_type = &reg.person_draft.person_type;
        let reg_model = self.registration.clone();

        div()
            .flex_row()
            .gap_2()
            .children(["employee", "guest"].iter().map(|t| {
                let t_str = t.to_string();
                let is_active = current_type == &t_str;

                div()
                    .px_2()
                    .py_1()
                    .bg(if is_active {
                        rgb(0x4a90e2)
                    } else {
                        rgb(0x333333)
                    })
                    .child(t_str.clone())
                    .on_mouse_down(MouseButton::Left, {
                        let reg_model = reg_model.clone();
                        let t_str = t_str.clone();
                        move |_, cx| {
                            reg_model.update(cx, |r, cx| {
                                r.update_access_level(t_str.clone());
                                cx.notify();
                            });
                        }
                    })
            }))
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
                    reg.handle_text_input(&ev.keystroke);
                    model_cx.notify();
                });
            })
            .flex_col()
            .size_full()
            .bg(rgb(DARK_MODE_COLOR))
            .child(self.render_tab_bar(cx))
            .child(div().flex_grow().p_4().child(match active_tab {
                ActiveTab::Settings => self.render_settings_tab(cx),
                ActiveTab::Persons => self.render_persons_tab(cx),
                ActiveTab::Employees => div().child("Employee List").into_any_element(),
            }))
    }
}
