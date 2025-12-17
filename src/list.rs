use crate::styles::*;
use gpui::*;

pub struct List {
    value: String,
}

impl List {
    pub fn new(value: String) -> Self {
        List { value }
    }
}

impl Render for List {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(rgb(LIST_COLOR))
            .text_color(rgb(PRIMARY_COLOR))
            .h(DefiniteLength::Fraction(0.2))
            .px_8()
            .w_full()
            .flex()
            .items_center()
            .justify_end()
            .child(format!("{}", self.value))
    }
}
