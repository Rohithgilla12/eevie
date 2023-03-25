// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use base64::encode;
use screenshots::Screen;
use image::ImageFormat;
use std::time::Duration;
use serde_json::json;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn capture_screenshot(id: &str) -> Result<String, String> {
    println!("id: {:?}", id);
    // Get the list of screens
    let screens = Screen::all().unwrap();

    // Check if there's at least one screen
    if screens.is_empty() {
        return Err("No screens found".to_string());
    }

    let converted_id = id.parse::<u32>().unwrap();
    println!("converted_id: {:?}", converted_id);

    // Capture the screenshot of the primary screen

    let selected_screen = screens.iter().find(|screen| screen.display_info.id == converted_id).unwrap();
    println!("selected_screen: {:?}", selected_screen);


    let image = selected_screen.capture().unwrap();

    let buffer = image.buffer();
    let screenshot_data = format!("data:image/png;base64,{}", base64::encode(buffer));

    Ok(screenshot_data)
}

#[tauri::command]
fn get_available_screens() -> Result<String, String> {
    let screens = Screen::all().unwrap();

    // Check if there's at least one screen
    if screens.is_empty() {
        return Err("No screens found".to_string());
    }

    println!("screens: {:?}", screens);

    let mut screen_list = Vec::new();
    for screen in screens {
        let screen_info = json!({
            "x": screen.display_info.x,
            "y": screen.display_info.y,
            "width": screen.display_info.width,
            "height": screen.display_info.height,
            "is_primary": screen.display_info.is_primary,
            "id": screen.display_info.id,
        });
        screen_list.push(screen_info);
    }

    let screen_list_json = json!(screen_list);
    Ok(screen_list_json.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,capture_screenshot, get_available_screens,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
