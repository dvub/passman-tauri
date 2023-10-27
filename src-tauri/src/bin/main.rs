// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{
    backend::{db_ops::util::authenticate, password::PasswordField},
    state::{self, AppState, ServiceAccess},
};
use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn auth(app_handle: AppHandle, master: &str, column: PasswordField) -> bool {
    app_handle
        .db(|db| authenticate(db, master, column))
        .unwrap()
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<AppState> = handle.state();
            let db = app::backend::db_ops::util::tauri::init_database(&handle)
                .expect("Database initialization should succeed");
            *app_state.db.lock().unwrap() = Some(db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
