use gpui::{
    div, rgb, DefiniteLength, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent,
    ParentElement, Render, Styled, ViewContext,
};
use gpui::{px, VisualContext};

use crate::button::*;
use crate::consts::*;
use crate::list::*;
use crate::logic::*;
use crate::styles::*;

pub struct Root {
    pub logic: Logic,
    focus_handle: FocusHandle,
}

impl Root {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let logic = Logic::new();

        Self {
            logic,
            focus_handle: cx.focus_handle(),
        }
    }

    fn get_buttons(&self, cx: &mut ViewContext<Self>) -> Vec<Button> {
        let mut buttons = Vec::new();

        for button_type in BUTTONS {
            let variant = match button_type {
                ButtonType::RegisterEmployee => ButtonVariant::Primary,
                ButtonType::Settings => ButtonVariant::Neutral,
                _ => ButtonVariant::Secondary,
            };

            let button =
                Button::new(button_type, variant).on_click(cx.listener(move |this, _view, cx| {
                    this.logic.on_button_pressed(button_type);
                    cx.notify()
                }));

            buttons.push(button);
        }

        buttons
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let list_placeholder = self.logic.get_list_value();
        let buttons = self.get_buttons(cx);

        // To accept key stroke events it is necessary to focus the
        // view at the beginning
        cx.focus(&self.focus_handle);

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, cx| {
                this.logic.handle_key_input(&event.keystroke.key.as_str());
                cx.notify();
            }))
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(BUTTON_PANEL_COLOR))
            .text_lg()
            .child(cx.new_view(|_cx| List::new(list_placeholder)))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_end()
                    .justify_start()
                    .h(DefiniteLength::Fraction(0.80))
                    .py(DefiniteLength::Fraction(0.02))
                    .gap(DefiniteLength::Fraction(0.02))
                    .children(buttons)
                    .ml(px(10.0)),
            )
    }
}
