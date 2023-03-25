// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::encode;
use image::{DynamicImage, ImageBuffer, Rgb, RgbImage, Rgba};
use imageproc::{
    definitions::Image,
    drawing::{draw_filled_rect_mut, Canvas},
    rect::Rect,
};
use screenshots::Screen;
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

    let selected_screen = screens
        .iter()
        .find(|screen| screen.display_info.id == converted_id)
        .unwrap();
    println!("selected_screen: {:?}", selected_screen);

    let image = selected_screen.capture().unwrap();
    let buffer = image.buffer();

    let updated_image = image::load_from_memory(buffer).unwrap();

    let (width, height) = updated_image.dimensions();

    println!("width: {:?}", width);
    println!("height: {:?}", height);

    let padding = 32;

    let mut new_img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(width + (padding * 2), height + (padding * 2));

    for x in 0..width {
        for y in 0..height {
            let pixel = updated_image.get_pixel(x, y);

            new_img.put_pixel(
                x + padding,
                y + padding,
                Rgb([pixel[0], pixel[1], pixel[2]]),
            );
        }
    }

    let radius = 24;
    let rect = Rect::at(radius as i32, radius as i32).of_size(width, height);

    new_img.save("../screenshot.png").unwrap();

    let screenshot_data = format!("data:image/png;base64,{}", base64::encode(buffer));

    Ok(screenshot_data)
}

#[tauri::command]
fn get_available_screens() -> Result<String, String> {
    let screens = Screen::all().unwrap();

    // Check if there's at least one screen:43

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
        .invoke_handler(tauri::generate_handler![
            greet,
            capture_screenshot,
            get_available_screens,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
