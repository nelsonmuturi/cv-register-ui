use gpui::*;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone, Copy)]
pub enum ButtonType {
    RegisterEmployee,
    Settings,
}

pub enum RegistrationEvent {
    OpenSettings,
    OpenRegister,
}

pub struct Registration {
    pub db_connected: bool,
    pub connection_error: Option<String>,
}

impl EventEmitter<RegistrationEvent> for Registration {}

impl Registration {
    pub fn new() -> Self {
        Self {
            db_connected: false,
            connection_error: None,
        }
    }

    pub fn handle_key_input(&mut self, key_input: &str, cx: &mut ModelContext<Self>) {
        match key_input {
            "enter" => self.on_button_pressed(ButtonType::RegisterEmployee, cx),
            "s" => self.on_button_pressed(ButtonType::Settings, cx),
            _ => {
                if key_input.parse::<u8>().is_ok() {
                    self.on_button_pressed(ButtonType::RegisterEmployee, cx);
                }
            }
        }
    }

    pub fn on_button_pressed(&mut self, button_type: ButtonType, cx: &mut ModelContext<Self>) {
        match button_type {
            ButtonType::RegisterEmployee => cx.emit(RegistrationEvent::OpenRegister),
            ButtonType::Settings => cx.emit(RegistrationEvent::OpenSettings),
        }
    }

    pub fn connect_to_db(&self, db_name: String, password: String, cx: &mut ModelContext<Self>) {
        // Construct connection string
        let url = format!("postgres://postgres:{}@localhost/{}", password, db_name);

        // Spawn an async task so the UI remains responsive
        cx.spawn(|this, mut cx| async move {
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .acquire_timeout(std::time::Duration::from_secs(3))
                .connect(&url)
                .await;

            // Update the model with the result
            this.update(&mut cx, |reg, cx| {
                match pool {
                    Ok(_) => {
                        reg.db_connected = true;
                        reg.connection_error = None;
                    }
                    Err(e) => {
                        reg.db_connected = false;
                        reg.connection_error = Some(e.to_string());
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}
