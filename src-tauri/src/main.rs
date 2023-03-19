// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use base64::encode;
use screenshots::Screen;
use image::ImageFormat;
use std::io::Cursor;
use std::thread;
use std::time::Duration;
use imageproc::drawing::Canvas;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn capture_screenshot() -> Result<String, String> {
    // Get the list of screens
    // let screens = get_screens()?;
    let screens = Screen::all().unwrap();

    // Check if there's at least one screen
    if screens.is_empty() {
        return Err("No screens found".to_string());
    }

    // Capture the screenshot of the primary screen
    let screen = Screen::from_point(0, 0).unwrap();

    let image = screen.capture_area(0,0, screen.display_info.width, screen.display_info.height).unwrap();
    let buffer = image.buffer();
    let screenshot_data = format!("data:image/png;base64,{}", base64::encode(buffer));

    Ok(screenshot_data)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,capture_screenshot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
