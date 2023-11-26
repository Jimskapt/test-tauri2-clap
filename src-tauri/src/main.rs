// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn default_name(default_name: tauri::State<String>) -> String {
    default_name.to_string()
}

fn main() {
    let cli = ClapApp::parse();

    tauri::Builder::default()
        .manage(cli.default_name)
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, default_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ClapApp {
    #[arg(
		long,
	)]
    default_name: String,
}
