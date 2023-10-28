// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod crypto;
pub mod db_ops;
pub mod error;
pub mod password;
pub mod state;

use db_ops::{
    MASTER_KEYWORD,
    {check_password_info_exists, tauri::init_database},
};
use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn check_master_exists(app_handle: AppHandle) -> bool {
    app_handle
        .db(|connection| check_password_info_exists(connection, MASTER_KEYWORD))
        .unwrap()
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
        .invoke_handler(tauri::generate_handler![check_master_exists])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
