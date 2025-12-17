use crate::styles::*;
use gpui::*;

pub struct Settings {
    db_name: String,
    password: String,
    // A callback to notify the parent of the result
    on_connect: Box<dyn Fn(bool, Option<String>, &mut WindowContext) + 'static>,
}

impl Settings {
    pub fn new(on_connect: impl Fn(bool, Option<String>, &mut WindowContext) + 'static) -> Self {
        Self {
            db_name: "face_identity_db".into(),
            password: "".into(),
            on_connect: Box::new(on_connect),
        }
    }
}

impl Render for Settings {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_col()
            .size_full()
            .bg(rgb(LIST_COLOR))
            .p_4()
            .gap_4()
            .child(div().child("Database Name:").text_color(rgb(WHITE_COLOR)))
            // Simple placeholder for an input field
            .child(
                div()
                    .bg(rgb(BUTTON_PANEL_COLOR))
                    .p_2()
                    .child(self.db_name.clone()),
            )
            .child(div().child("Password:").text_color(rgb(WHITE_COLOR)))
            .child(div().bg(rgb(BUTTON_PANEL_COLOR)).p_2().child("********"))
            .child(
                div()
                    .bg(rgb(PRIMARY_COLOR))
                    .p_2()
                    .child("Connect to Database")
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, move |_, cx| {
                        // Logic: Mocking a connection check
                        let success = true;
                        (self.on_connect)(success, None, cx);
                    }),
            )
    }
}
