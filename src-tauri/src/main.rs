#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use co_ddl::{async_sign_in, async_sign_up};
use tauri::async_runtime::block_on;

// const ADDR: &str = "[2001:da8:c800:b20c:77ca:7dd0:6e36:168d]:8848";

#[tauri::command]
fn sign_in(username: &str, password: &str) -> bool {
    block_on(async_sign_in(username, password))
}

#[tauri::command]
fn sign_up(username: &str, password: &str, email: &str) -> bool {
    block_on(async_sign_up(username, password, email))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sign_in, sign_up])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
