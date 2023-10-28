// source: https://github.com/RandomEngy/tauri-sqlite
// i tried to comment this code to help me better understand everything that's happening. so excuse the messy comments.

use rusqlite::Connection;
use tauri::{AppHandle, Manager, State};

// i'm not very good at rust, so this helped me understand mutex's (is that the right plural?)
// https://fongyoong.github.io/easy_rust/Chapter_43.html
pub struct AppState {
    // Question: why is the connection optional?
    pub connection: std::sync::Mutex<Option<Connection>>,
}

// my big question is why we need the mutable functions?
// i'm not sure if any of my database operations actually need to mutate a connection ...

// I'm also not very familiar with traits so this is pretty foreign to me
pub trait ServiceAccess {
    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Connection) -> TResult;

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Connection) -> TResult,
    {
        //
        let app_state: State<AppState> = self.state();
        // the guard locks the data and prevents any other mutation (or access...?)
        let connection_guard = app_state.connection.lock().unwrap();
        // now we can actually get the connection, since we've locked it.
        let connection = connection_guard.as_ref().unwrap();

        // this is actually pretty intuitive
        operation(connection)
    }

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let mut db_connection_guard = app_state.connection.lock().unwrap();
        let db = db_connection_guard.as_mut().unwrap();

        operation(db)
    }
}
