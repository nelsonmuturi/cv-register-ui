#[derive(Clone, Copy)]
pub enum ButtonType {
    RegisterEmployee,
    Settings,
}

pub struct Registration {
    pub db_connected: bool,
    pub connection_error: Option<String>,
}

impl Registration {
    pub fn new() -> Self {
        Self {
            db_connected: false,
            connection_error: None,
        }
    }

    // Called by settings dialog
    pub fn set_connection_status(&mut self, success: bool, error: Option<String>) {
        self.db_connected = success;
        self.connection_error = error;
    }

    pub fn handle_key_input(&mut self, key_input: &str) {
        if let Ok(num) = key_input.parse::<u8>() {
            self.on_button_pressed(ButtonType::RegisterEmployee)
        } else {
            match key_input {
                "enter" => self.on_button_pressed(ButtonType::RegisterEmployee),
                "s" => self.on_button_pressed(ButtonType::Settings),
                _ => {}
            }
        }
    }

    pub fn get_list_value(&self) -> String {
        String::from("placeholder for list")
    }

    pub fn on_button_pressed(&mut self, button_type: ButtonType) {
        match button_type {
            ButtonType::RegisterEmployee => self.open_register_dialog(),
            ButtonType::Settings => self.open_settings_dialog(),
        }
    }

    fn open_register_dialog(&mut self) {
        // TODO
    }

    fn open_settings_dialog(&mut self) {
        // TODO
    }
}
