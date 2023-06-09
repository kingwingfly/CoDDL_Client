#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use co_ddl::{async_sign_in, async_sign_up};
use tauri::async_runtime::block_on;

#[tauri::command]
fn sign_in(username: &str, password: &str) -> bool {
    block_on(async_sign_in(username, password))
}

#[tauri::command]
fn sign_up(username: &str, password: &str, email: &str) -> bool {
    block_on(async_sign_up(username, password, email))
}

#[tauri::command]
fn server_address() -> String {
    co_ddl::ADDR.to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sign_in, sign_up, server_address])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
