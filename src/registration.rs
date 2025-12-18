use gpui::*;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone, Copy, PartialEq)]
pub enum ActiveTab {
    Settings,
    Manage,
    Employees,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FocusField {
    DbName,
    DbPassword,
    None,
}

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
    pub active_tab: ActiveTab,
    pub db_connected: bool,
    pub db_name: String,
    pub db_password: String,
    pub focused_field: FocusField, // Track which field is being typed into
    pub connection_error: Option<String>,
}

impl EventEmitter<RegistrationEvent> for Registration {}

impl Registration {
    pub fn new() -> Self {
        Self {
            active_tab: ActiveTab::Settings, // Default tab
            db_connected: false,
            db_name: "face_identity_db".into(),
            db_password: "".into(),
            focused_field: FocusField::None,
            connection_error: None,
        }
    }

    pub fn set_tab(&mut self, tab: ActiveTab) {
        self.active_tab = tab;
    }

    pub fn handle_text_input(&mut self, key: &str) {
        let target = match self.focused_field {
            FocusField::DbName => &mut self.db_name,
            FocusField::DbPassword => &mut self.db_password,
            FocusField::None => return,
        };

        match key {
            "backspace" => {
                target.pop();
            }
            "space" => {
                target.push(' ');
            }
            k if k.len() == 1 => {
                target.push_str(k);
            }
            _ => {}
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
