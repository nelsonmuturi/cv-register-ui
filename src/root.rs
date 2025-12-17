use gpui::{
    div, rgb, size, Bounds, Context, DefiniteLength, FocusHandle, InteractiveElement, IntoElement,
    KeyDownEvent, Model, ParentElement, Render, Styled, TitlebarOptions, ViewContext, WindowBounds,
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
    pub registration: Model<Registration>,
    focus_handle: FocusHandle,
}

impl Root {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        // Create the model in the app context
        let registration = cx.new_model(|_| Registration::new());

        // 3. Subscribe to model events
        cx.subscribe(&registration, |this, _model, event, cx| match event {
            RegistrationEvent::OpenSettings => this.open_settings_dialog(cx),
            RegistrationEvent::OpenRegister => {
                println!("Open Register Dialog (TODO)");
            }
        })
        .detach(); // Keep the subscription alive for the life of the view

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
                    // Correct way to call a method on a Model
                    this.registration.update(cx, |reg, cx| {
                        reg.on_button_pressed(button_type, cx);
                    });
                }));

            buttons.push(button);
        }

        buttons
    }

    pub fn open_settings_dialog(&mut self, cx: &mut ViewContext<Self>) {
        let registration_model = self.registration.clone();
        let bounds = Bounds::centered(None, size(px(400.0), px(200.0)), cx);

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |cx| cx.new_view(|_| Settings::new(registration_model)),
        );
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let buttons = self.get_buttons(cx);

        // Read the state from the model
        let reg = self.registration.read(cx);
        let status_color = if reg.db_connected {
            STATUS_COLOR_GREEN
        } else {
            STATUS_COLOR_RED
        };
        let status_text = if reg.db_connected {
            "DB: Connected"
        } else {
            "DB: Disconnected"
        };

        // To accept key stroke events it is necessary to focus the
        // view at the beginning
        cx.focus(&self.focus_handle);

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, cx| {
                let key = event.keystroke.key.as_str();

                // Use update to access the Registration methods
                this.registration.update(cx, |reg, cx| {
                    reg.handle_key_input(key, cx); // Pass cx to trigger events
                });

                // If handle_key_input triggers a state change (like opening a window),
                // we handle that logic in the model's update or via a listener.
                cx.notify();
            }))
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(BUTTON_PANEL_COLOR))
            .text_lg()
            .text_sm()
            .text_color(rgb(status_color))
            .child(
                // Status Indicator at the top
                div()
                    .p_2()
                    .text_sm()
                    .text_color(rgb(status_color))
                    .child(status_text),
            )
            .child(cx.new_view(|_cx| List::new("Employee List".to_string())))
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
