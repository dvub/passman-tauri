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
fn read_password(
    app_handle: AppHandle,
    search_term: &str,
    master: &str,
) -> Result<Option<PasswordInfo>, BackendError> {
    app_handle.db(|connection| read_password_info(connection, search_term, master))
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
        .invoke_handler(tauri::generate_handler![read_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
