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

pub struct Registration {
    pub active_tab: ActiveTab,
    pub db_connected: bool,
    pub db_name: String,
    pub db_password: String,
    pub focused_field: FocusField, // Track which field is being typed into
    pub connection_error: Option<String>,
}

impl Registration {
    pub fn new() -> Self {
        Self {
            active_tab: ActiveTab::Settings, // Default tab
            db_connected: false,
            db_name: "face_identity_db".into(),
            db_password: "m68U0Qd2pZ".into(),
            focused_field: FocusField::None,
            connection_error: None,
        }
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
            // Use a more robust check for single characters (including symbols and caps)
            k if k.chars().count() == 1 => {
                target.push_str(k);
            }
            _ => {}
        }
    }

    pub fn connect_to_db(&self, db_name: String, password: String, cx: &mut ModelContext<Self>) {
        println!("db_name {db_name}, password {password}");

        // Construct connection string
        let url = format!("postgres://postgres:{}@localhost/{}", password, db_name);

        // 1. Capture the current Tokio handle from the main thread
        let tokio_handle = tokio::runtime::Handle::current();

        // Spawn an async task so the UI remains responsive
        cx.spawn(|this, mut cx| async move {
            // 2. "Enter" the Tokio context so sqlx can find the reactor
            let pool = {
                let _guard = tokio_handle.enter();
                PgPoolOptions::new()
                    .max_connections(5)
                    .acquire_timeout(std::time::Duration::from_secs(3))
                    .connect(&url)
                    .await
            };

            // 3. Jump back into GPUI's update loop to show the result
            this.update(&mut cx, |reg, cx| {
                match pool {
                    Ok(_) => {
                        reg.db_connected = true;
                        reg.connection_error = None;
                        println!("Connected successfully!");
                    }
                    Err(e) => {
                        reg.db_connected = false;
                        reg.connection_error = Some(e.to_string());
                        eprintln!("Connection failed: {}", e);
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}
