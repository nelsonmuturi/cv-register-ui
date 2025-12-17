use crate::registration::*;
use crate::styles::*;
use gpui::*;

pub struct Settings {
    registration: Model<Registration>,
}

impl Settings {
    pub fn new(registration: Model<Registration>) -> Self {
        Self { registration }
    }
}

impl Render for Settings {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let registration = self.registration.clone();

        div()
            .flex_col()
            .size_full()
            .bg(rgb(LIST_COLOR))
            .p_4()
            .gap_4()
            .child(
                div()
                    .child("Database: face_identity_db")
                    .text_color(rgb(WHITE_COLOR)),
            )
            .child(
                div()
                    .bg(rgb(PRIMARY_COLOR))
                    .p_2()
                    .rounded_md()
                    .child("Connect Now")
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, move |_, cx| {
                        registration.update(cx, |reg, cx| {
                            // In a real app, you'd pull the actual strings from your input fields
                            reg.connect_to_db(
                                "Connect DB".to_string(),
                                "your_password".to_string(),
                                cx,
                            );
                        });
                        cx.remove_window();
                    }),
            )
    }
}
