// ANCHOR: example
//! Create a simple Tauri app with a menu and counter command handlers.
//!
//! This example defines backend commands and opens a Tauri window using the
//! generated application context.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::CustomMenuItem;
use tauri::Manager;
use tauri::Menu;
use tauri::MenuItem;
use tauri::State;
use tauri::StateMut;
use tauri::Submenu;

#[tauri::command]
fn get_counter(counter: State<'_, CounterState>) -> i32 {
    counter.value
}

#[tauri::command]
fn increment_counter(mut counter: StateMut<'_, CounterState>) {
    counter.value += 1;
}

#[tauri::command]
fn decrement_counter(mut counter: StateMut<'_, CounterState>) {
    counter.value -= 1;
}

struct CounterState {
    value: i32,
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let submenu = Submenu::new("File", Menu::new().add_item(quit));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                event.window().hide().unwrap();
            }
            _ => {}
        })
        .manage(CounterState { value: 0 })
        .invoke_handler(tauri::generate_handler![
            get_counter,
            increment_counter,
            decrement_counter,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_main() {
        main();
    }
}
