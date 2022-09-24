#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod deserialization;
pub mod projects;
pub mod state;
pub mod commands;

use state::create_storytell_state;
use commands::*;

fn main() {
  let state = create_storytell_state();
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![list_projects, create_project, delete_project, edit_project, init_compiler, rename_blob, delete_blob, create_blob, refresh_blobs, recompile_file, save_data])
    .menu(if cfg!(target_os = "macos") {
      tauri::Menu::os_default(&context.package_info().name)
    } else {
      tauri::Menu::default()
    })
    .run(context)
    .expect("error while running tauri application");
}
