#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod commands;
use tauri_plugin_store::{PluginBuilder, StoreBuilder};

fn main() {
  tauri::Builder::default()
  .plugin(
    PluginBuilder::default()
      .build(),
  )
  .run(tauri::generate_context!())
  .expect("failed to run app");
}
