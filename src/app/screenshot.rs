use std::fs;
use std::path::PathBuf;

use bevy::render::view::window::screenshot::ScreenshotManager;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, render::texture::*};

const SCREENSHOT_TYPES: [&str; 2] = ["score", "max_alpha_count"];

pub fn shot_current(
    main_window: &Query<Entity, With<PrimaryWindow>>,
    screenshot_manager: &mut ResMut<ScreenshotManager>,
    screenshot_type: &str,
) {
    if SCREENSHOT_TYPES.contains(&screenshot_type) {
        let screenshot_dir = fetch_screenshots_dir_path();
        if !screenshot_dir.exists() {
            let _ = fs::create_dir_all(&screenshot_dir);
        }
        let path = build_screenshot_file_path("current", screenshot_type);
        let _ = screenshot_manager.save_screenshot_to_disk(main_window.single(), path);
    } else {
        panic!("Invalid screenshot type")
    }
}

pub fn store_leaderboard_screenshots(uid: &str) {
    for screenshot_type in SCREENSHOT_TYPES {
        let src_path = build_screenshot_file_path("current", screenshot_type);
        let dest_path = build_screenshot_file_path(uid, screenshot_type);
        if src_path.exists() {
            let _ = fs::copy(src_path, dest_path);
        }
    }
}

pub fn fetch_screenshot_image(uid: &str, screenshot_type: &str) -> Image {
    let image_path = build_screenshot_file_path(uid, screenshot_type);
    let image_data = fs::read(image_path).unwrap();
    Image::from_buffer(
        &image_data,
        ImageType::Format(ImageFormat::Png),
        CompressedImageFormats::default(),
        true,
        ImageSampler::default(),
    )
    .unwrap()
}

fn fetch_screenshots_dir_path() -> PathBuf {
    dirs::config_dir()
        .map(|native_config_dir| native_config_dir.join("chain-reaction").join("screenshots"))
        .unwrap()
}

fn build_screenshot_file_path(screenshot_uid: &str, screenshot_type: &str) -> PathBuf {
    let screenshot_dir = fetch_screenshots_dir_path();
    screenshot_dir.join(format!("{}_{}.png", screenshot_uid, screenshot_type))
}
