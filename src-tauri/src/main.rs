#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod commands;
use tauri_plugin_store::{PluginBuilder, StoreBuilder};

fn main() {
  let settings = StoreBuilder::new("wallpapers_data.bin".parse().unwrap())
    .default("the-key".to_string(), "wooooot".into())
    .build();

  tauri::Builder::default()
    .plugin(PluginBuilder::default().stores([settings]).freeze().build())
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
