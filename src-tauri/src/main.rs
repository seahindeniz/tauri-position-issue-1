#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let tray_menu = SystemTrayMenu::new().add_item(quit);

  tauri::Builder::default()
    .system_tray(SystemTray::new().with_menu(tray_menu))
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { position, size, .. } => {
        let window = app.get_window("main").unwrap();
        let current_monitor = window.current_monitor().unwrap().unwrap();
        let monitor_position = current_monitor.position();
        let monitor_size = current_monitor.size();

        println!("Tray position {} {}", position.x, position.y);
        println!("Tray size {} {}", size.width, size.height);
        println!(
          "\nWindow position {} {}",
          monitor_position.x, monitor_position.y
        );
        println!("Window size {} {}", monitor_size.width, monitor_size.height);
        println!("-----")
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
