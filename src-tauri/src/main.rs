// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use std::path::PathBuf;


fn main() {
//   list_ssh_keys();
  app_lib::run();
  tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![list_ssh_keys, my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
