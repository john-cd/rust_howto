// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// // (backend)

// // Project Setup: Create a new Tauri project:
// // cargo create --bin my-tauri-app
// // cd my-tauri-app
// // tauri init

// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// #[tauri::command]
// fn get_counter(counter: tauri::State<'_, CounterState>) -> i32 {
//     counter.value
// }

// #[tauri::command]
// fn increment_counter(counter: tauri::StateMut<'_, CounterState>) {
//     counter.value += 1;
// }

// #[tauri::command]
// fn decrement_counter(counter: tauri::StateMut<'_, CounterState>) {
//     counter.value -= 1;
// }

// struct CounterState {
//     value: i32,
// }

// pub fn main() {
//         let quit = CustomMenuItem::new("quit".to_string(), "Quit");
//     let submenu = Submenu::new("File", Menu::new().add_item(quit));
//     let menu = Menu::new()
//         .add_native_item(MenuItem::Copy)
//         .add_item(CustomMenuItem::new("hide", "Hide"))
//         .add_submenu(submenu);

//     tauri::Builder::default()
//         .menu(menu)
//         .on_menu_event(|event| {
//             match event.menu_item_id() {
//                 "quit" => {
//                     std::process::exit(0);
//                 }
//                 "hide" => {
//                     event.window().hide().unwrap();
//                 }
//                 _ => {}
//             }
//         })
//         .manage(CounterState { value: 0 })
//         .invoke_handler(tauri::generate_handler![
//             get_counter,
//             increment_counter,
//             decrement_counter
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

pub fn main() {}
// // [P1](https://github.com/john-cd/rust_howto/issues/790)
