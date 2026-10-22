//! A desktop application with a Rust backend, built with `tauri`.
//!
//! TODO Reviewed ??-??-2026

// ANCHOR: example
// (backend)
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use tauri::Manager;
use tauri::menu::MenuBuilder;
use tauri::menu::MenuItemBuilder;

/// Returns the current value of the counter.
#[tauri::command]
fn get_counter(state: tauri::State<'_, CounterState>) -> i32 {
    *state.value.lock().unwrap()
}

/// Increments the counter.
#[tauri::command]
fn increment_counter(state: tauri::State<'_, CounterState>) {
    *state.value.lock().unwrap() += 1;
}

/// The state of the counter, managed by Tauri.
struct CounterState {
    /// The current value.
    value: Mutex<i32>,
}

/// Runs the application.
pub fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&quit]).build()?;
            app.set_menu(menu)?;

            app.on_menu_event(move |app_handle, event| {
                if event.id().as_ref() == "quit" {
                    app_handle.exit(0);
                }
            });

            app.manage(CounterState {
                value: Mutex::new(0),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_counter,
            increment_counter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Runs the example.
pub fn run() {
    main();
}
// ANCHOR_END: example
