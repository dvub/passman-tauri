// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod crypto;
pub mod db_ops;
pub mod error;
pub mod password;
pub mod state;

use db_ops::{
    crud_operations::{insert_data, read_password_info},
    tauri::init_database,
};
use error::BackendError;
use password::{PasswordField, PasswordInfo};
use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn execute_function(
    app_handle: AppHandle,
    function: SqlFunction,
) -> Result<serde_json::Value, BackendError> {
    let result = app_handle.db(|connection| match function {
        SqlFunction::ReadPasswordInfo {
            search_term,
            master,
        } => {
            let p = read_password_info(connection, &search_term, &master)?;
            serde_json::Value::from(p);
            serde_json::from_value(p)
        }
    });
}

enum SqlFunction {
    ReadPasswordInfo { search_term: String, master: String },
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            connection: Default::default(),
        })
        // setup function:
        // this is where we can do any database connection, setup, upgrades, etc.
        // m
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<AppState> = handle.state();

            // create our database connection, as well as inserting our table, etc.
            let connection =
                init_database(&handle).expect("Database initialization should succeed");
            // setting the state's `connection` field to the one we just initialized.

            *app_state.connection.lock().unwrap() = Some(connection);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
