use gpui::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

#[derive(Clone, Copy, PartialEq)]
pub enum ActiveTab {
    Settings,
    Persons,
    Employees,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FocusField {
    DbName,
    DbPassword,
    PersonName,
    None,
}

#[derive(Clone, Copy)]
pub enum ButtonType {
    Settings,
    Persons,
    Employees,
    Insert,
    Edit,
    Delete,
    Connect,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
pub struct Person {
    pub person_id: i32,
    pub full_name: String,
    pub person_type: String,
    pub access_level: String,
}

// State for the 'Manage' form
pub struct PersonDraft {
    pub full_name: String,
    pub person_type: String,
    pub access_level: String,
}

impl Default for PersonDraft {
    fn default() -> Self {
        Self {
            full_name: "".into(),
            person_type: "employee".into(),  // Default
            access_level: "employee".into(), // Default
        }
    }
}

pub struct Registration {
    pub active_tab: ActiveTab,
    pub db_connected: bool,
    pub db_pool: Option<sqlx::PgPool>,
    pub db_name: String,
    pub db_password: String,
    pub focused_field: FocusField, // Track which field is being typed into
    pub connection_error: Option<String>,
    pub persons: Vec<Person>,
    pub selected_person: Option<Person>,
    pub person_draft: PersonDraft,
    pub is_loading: bool,
}

impl Registration {
    pub fn new() -> Self {
        Self {
            active_tab: ActiveTab::Settings, // Default tab
            db_connected: false,
            db_pool: None,
            db_name: "face_identity_db".into(),
            db_password: "m68U0Qd2pZ".into(),
            focused_field: FocusField::None,
            connection_error: None,
            persons: Vec::new(),
            selected_person: None,
            person_draft: PersonDraft {
                full_name: "".into(),
                person_type: "employee".into(),
                access_level: "employee".into(),
            },
            is_loading: false,
        }
    }

    pub fn handle_text_input(&mut self, key: &str) {
        let target = match self.focused_field {
            FocusField::DbName => &mut self.db_name,
            FocusField::DbPassword => &mut self.db_password,
            FocusField::PersonName => &mut self.person_draft.full_name,
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
                    Ok(p) => {
                        reg.db_connected = true;
                        reg.db_pool = Some(p);
                        reg.connection_error = None;
                        println!("Connected successfully!");
                    }
                    Err(e) => {
                        reg.db_connected = false;
                        reg.db_pool = None;
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

    // gem-2026-01-07: Fetch all persons from database
    pub fn fetch_persons(&mut self, cx: &mut ModelContext<Self>) {
        // Ensure we have a pool to work with
        let Some(pool) = self.db_pool.clone() else {
            return;
        };

        self.is_loading = true;
        cx.notify();

        cx.spawn(|this, mut cx| async move {
            let results = sqlx::query_as::<_, Person>(
                "SELECT person_id, full_name, person_type, access_level FROM persons ORDER BY created_at DESC"
            ).fetch_all(&pool)
            .await;

            this.update(&mut cx, |reg, cx| {
                if let Ok(data) = results {
                    reg.persons = data;
                } else if let Err(e) = results {
                    eprintln!("Query error: {}",  e)
                }
                reg.is_loading = false;
                cx.notify();
            }).ok();
        }).detach();
    }

    pub fn save_person(&mut self, cx: &mut ModelContext<Self>) {
        let Some(pool) = self.db_pool.clone() else {
            return;
        };

        let name = self.person_draft.full_name.clone();
        let p_type = self.person_draft.person_type.clone();
        let access = self.person_draft.access_level.clone();

        cx.spawn(|this, mut cx| async move {
            let res = sqlx::query(
                "INSERT INTO persons (full_name, person_type, access_level) VALUES ($1, $2, $3)",
            )
            .bind(name)
            .bind(p_type)
            .bind(access)
            .execute(&pool)
            .await;

            this.update(&mut cx, |reg, cx| {
                if res.is_ok() {
                    reg.person_draft = PersonDraft::default(); // Reset form [cite: 76, 151]
                    reg.fetch_persons(cx); // Refresh the list [cite: 77]
                } else if let Err(e) = res {
                    eprintln!("Save error: {}", e);
                }
                cx.notify(); // [cite: 56]
            })
            .ok();
        })
        .detach();
    }
}
