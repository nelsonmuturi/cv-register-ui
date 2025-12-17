use gpui::{
    div, rgb, size, Bounds, DefiniteLength, FocusHandle, InteractiveElement, IntoElement,
    KeyDownEvent, ParentElement, Render, Styled, TitlebarOptions, ViewContext, WindowBounds,
    WindowOptions,
};
use gpui::{px, VisualContext};

use crate::button::*;
use crate::consts::*;
use crate::list::*;
use crate::registration::*;
use crate::settings::*;
use crate::styles::*;

pub struct Root {
    pub registration: Registration,
    focus_handle: FocusHandle,
}

impl Root {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let registration = Registration::new();

        Self {
            registration,
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
                    this.registration.on_button_pressed(button_type);
                    cx.notify()
                }));

            buttons.push(button);
        }

        buttons
    }

    fn open_settings_dialog(&mut self, cx: &mut ViewContext<Self>) {
        let bounds = Bounds::centered(None, size(px(400.0), px(300.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Database Settings".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| {
                    Settings::new(|success, error, cx| {
                        // This closure needs to update the Root's registration state
                        // However, updating across windows usually requires a Global or Model.
                        // For a simple POC, you can use cx.update_global or emit an event.
                        println!("Connection result: {}", success);
                    })
                })
            },
        );
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let list_placeholder = self.registration.get_list_value();
        let buttons = self.get_buttons(cx);

        // To accept key stroke events it is necessary to focus the
        // view at the beginning
        cx.focus(&self.focus_handle);

        let status_text = if self.registration.db_connected {
            "Connected"
        } else {
            "Not Connected"
        };

        let status_color = if self.registration.db_connected {
            STATUS_COLOR_GREEN
        } else {
            STATUS_COLOR_RED
        };

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, cx| {
                this.registration
                    .handle_key_input(&event.keystroke.key.as_str());
                cx.notify();
            }))
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(BUTTON_PANEL_COLOR))
            .text_lg()
            .text_sm()
            .text_color(rgb(status_color))
            .child(status_text)
            .child(cx.new_view(|_cx| List::new(list_placeholder)))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_end()
                    .justify_start()
                    .h(DefiniteLength::Fraction(0.15))
                    .py(DefiniteLength::Fraction(0.02))
                    .gap(DefiniteLength::Fraction(0.02))
                    .children(buttons)
                    .ml(px(10.0)),
            )
    }
}
