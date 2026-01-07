use crate::registration::*;
use crate::styles::*;
use gpui::*;

pub enum ButtonVariant {
    Primary,
    Secondary,
    Neutral,
}

struct ButtonStyle {
    bg: u32,
    text_color: u32,
    hover_color: u32,
}

#[derive(IntoElement)]
pub struct Button {
    on_click: Box<dyn Fn(&MouseDownEvent, &mut WindowContext) + 'static>,
    base: Div,
    text: String,
    variant: ButtonVariant,
}

impl Button {
    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Box::new(handler);
        self
    }

    fn get_variant_style(&self) -> ButtonStyle {
        match self.variant {
            ButtonVariant::Neutral => ButtonStyle {
                bg: BUTTON_PANEL_COLOR,
                text_color: WHITE_COLOR,
                hover_color: BUTTON_COLOR_HOVER,
            },
            ButtonVariant::Primary => ButtonStyle {
                bg: PRIMARY_COLOR,
                text_color: WHITE_COLOR,
                hover_color: PRIMARY_DARK,
            },
            ButtonVariant::Secondary => ButtonStyle {
                bg: BUTTON_COLOR,
                text_color: PRIMARY_COLOR,
                hover_color: BUTTON_COLOR_HOVER,
            },
        }
    }

    fn get_label(button_type: &ButtonType) -> String {
        match button_type {
            ButtonType::Settings => "Settings".to_owned(),
            ButtonType::Persons => "Persons".to_owned(),
            ButtonType::Employees => "List".to_owned(),
            ButtonType::Insert => "Save".to_owned(),
            ButtonType::Edit => "Edit".to_owned(),
            ButtonType::Delete => "Delete".to_owned(),
            ButtonType::Connect => "Connect to Postgres DB".to_owned(),
        }
    }
}

impl Button {
    pub fn new(button_type: ButtonType, variant: ButtonVariant) -> Self {
        Self {
            on_click: Box::new(|_event, _cx| {}),
            base: div(),
            text: Self::get_label(&button_type),
            variant,
        }
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let style = self.get_variant_style();

        self.base
            .cursor_pointer()
            .h(DefiniteLength::Fraction(1.0))
            .bg(rgb(style.bg))
            .text_color(rgb(style.text_color))
            .hover(|this| this.bg(rgb(style.hover_color)))
            .flex()
            .p(px(10.0))
            .rounded_lg()
            .items_center()
            .justify_center()
            .child(self.text)
            .on_mouse_down(MouseButton::Left, move |event, cx| {
                (&self.on_click)(event, cx)
            })
    }
}
