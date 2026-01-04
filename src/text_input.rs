use crate::styles::*;
use gpui::*;

#[derive(IntoElement)]
pub struct TextInput {
    label: String,
    value: String,
    is_focused: bool,
    is_password: bool,
    on_click: Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>,
}

impl TextInput {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            is_focused: false,
            is_password: false,
            on_click: Box::new(|_, _| {}),
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    pub fn password(mut self, is_password: bool) -> Self {
        self.is_password = is_password;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Box::new(handler);
        self
    }
}

impl RenderOnce for TextInput {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let display_value = if self.is_password {
            "*".repeat(self.value.len())
        } else if self.value.is_empty() {
            " ".to_string() // Keep height if empty
        } else {
            self.value.clone()
        };

        div()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(WHITE_COLOR))
                    .child(self.label),
            )
            .child(
                div()
                    .p_2()
                    .bg(rgb(LIST_COLOR))
                    .text_color(rgb(WHITE_COLOR))
                    .border_2()
                    .rounded_md()
                    .hover(|this| this.bg(rgb(BUTTON_COLOR_HOVER)))
                    .cursor_text()
                    .border_color(if self.is_focused {
                        rgb(PRIMARY_COLOR)
                    } else {
                        rgb(BUTTON_COLOR)
                    })
                    .child(display_value)
                    .on_mouse_down(MouseButton::Left, move |ev, cx| (self.on_click)(ev, cx)),
            )
    }
}
