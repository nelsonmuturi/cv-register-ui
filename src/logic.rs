#[derive(Clone, Copy)]
pub enum Operation {
    Division,
    Times,
    Minus,
    Plus,
}

#[derive(Clone, Copy)]
pub enum ButtonType {
    RegisterEmployee,
    Settings,
}

pub struct Logic {
    first_value: f64,
    second_value: Option<f64>,
    operation: Option<Operation>,
    use_comma: bool,
}

impl Logic {
    pub fn new() -> Self {
        Self {
            first_value: 0.0,
            second_value: None,
            operation: None,
            use_comma: false,
        }
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
