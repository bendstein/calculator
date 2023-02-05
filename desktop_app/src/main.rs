#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, LogicalSize};

const WIDTH: i32 = 400;
const HEIGHT: i32 = 500;

fn main() -> Result<(), tauri::Error> {
  let builder = tauri::Builder::default()
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();
      //main_window.set_resizable(false)?;
      main_window.set_size(LogicalSize::new(WIDTH, HEIGHT))?;
      Ok(())
    });

  builder.run(tauri::generate_context!()) 
}
