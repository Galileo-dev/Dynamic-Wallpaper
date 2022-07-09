#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod commands;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use log::{info, trace, warn};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};
use tauri_plugin_store::{PluginBuilder, StoreBuilder};

fn main() {
  let tray_menu1 = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("show", "show"))
    .add_item(CustomMenuItem::new("exit_app", "Quit"));

  let system_tray = SystemTray::new().with_menu(tray_menu1);
  let colors = ColoredLevelConfig::default();
  let logger = LoggerBuilder::new()
    .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
    .with_colors(colors)
    .build();

  let app = tauri::Builder::default()
    .plugin(PluginBuilder::default().build())
    .plugin(logger)
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        // get a handle to the clicked menu item
        // note that `tray_handle` can be called anywhere,
        // just get a `AppHandle` instance with `app.handle()` on the setup hook
        // and move it to another function or thread
        let item_handle = app.tray_handle().get_item(&id);
        match id.as_str() {
          "show" => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
          }
          "exit_app" => {
            let window = app.get_window("main").unwrap();
            window.close();
            std::process::exit(0);
          }
          _ => {}
        }
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      commands::set_wallpaper,
      commands::decode_heic
    ])
    .build(tauri::generate_context!())
    .expect("failed to run app");

  app.run(|app_handle, event| match event {
    tauri::RunEvent::ExitRequested { api, .. } => {
      api.prevent_exit();
    }
    _ => {}
  })
}
