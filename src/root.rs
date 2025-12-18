use gpui::*;

use crate::button::*;
use crate::consts::*;
use crate::list::*;
use crate::registration::*;
use crate::settings::*;
use crate::styles::*;

pub struct Root {
    pub registration: Model<Registration>,
    focus_handle: FocusHandle,
}

impl Root {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        // Create the model in the app context
        let registration = cx.new_model(|_| Registration::new());

        Self {
            registration,
            focus_handle: cx.focus_handle(),
        }
    }

    fn render_tab_bar(&self, _cx: &mut ViewContext<Self>, active: ActiveTab) -> impl IntoElement {
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
            .child(self.tab_button(
                "Manage Employees",
                ActiveTab::Manage,
                active == ActiveTab::Manage,
            ))
            .child(self.tab_button(
                "Employee List",
                ActiveTab::Employees,
                active == ActiveTab::Employees,
            ))
    }

    fn tab_button(&self, label: &'static str, tab: ActiveTab, is_active: bool) -> impl IntoElement {
        let registration = self.registration.clone();
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
            .on_mouse_down(MouseButton::Left, move |_, cx| {
                registration.update(cx, |reg, cx| {
                    reg.set_tab(tab);
                    cx.notify();
                });
            })
    }

    fn render_settings_tab(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        let reg = self.registration.read(cx);

        // Clone for the first input field's closure
        let reg_for_name = self.registration.clone();
        // Clone for the second input field's closure
        let reg_for_pass = self.registration.clone();

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
            .child(
                div()
                    .child(format!("Status: {}", status_text))
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
                    .bg(rgb(PRIMARY_COLOR))
                    .p_2()
                    .rounded_md()
                    .child("Connect to Postgres")
                    .on_mouse_down(MouseButton::Left, move |_, cx| {
                        // Call your connect_to_db logic here
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
            .child(div().text_sm().child(label))
            .child(
                div()
                    .p_2()
                    .bg(rgb(LIST_COLOR))
                    .border_2()
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
            .bg(rgb(BUTTON_COLOR))
            .p_2()
            .rounded_md()
            .child(label)
            .cursor_pointer()
    }

    fn render_employees_tab(&self, _cx: &mut ViewContext<Self>) -> AnyElement {
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
        let reg = self.registration.read(cx);
        let active_tab = reg.active_tab;

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, cx| {
                let key = event.keystroke.key.as_str();

                this.registration.update(cx, |reg, cx| {
                    // If a field is focused, send input there
                    if reg.focused_field != FocusField::None {
                        reg.handle_text_input(key);
                    } else {
                        // Otherwise, handle tab switching/shortcuts
                        reg.handle_key_input(key, cx);
                    }
                });
                cx.notify();
            }))
            .size_full()
            .flex_col()
            .bg(rgb(BUTTON_PANEL_COLOR))
            .child(self.render_tab_bar(cx, active_tab)) // Top Nav
            .child(div().size_full().p_4().child(match active_tab {
                ActiveTab::Settings => self.render_settings_tab(cx),
                ActiveTab::Manage => self.render_manage_tab(cx),
                ActiveTab::Employees => self.render_employees_tab(cx),
            }))
    }
}
