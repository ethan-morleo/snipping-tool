pub(crate) mod command{
use std::path::Path;
use egui::{Pos2, Ui, Vec2};
use native_dialog::FileDialog;
use crate::app::app_utils::MyApp;
use crate::enums::app_enums::{RequestState};

pub fn new_screen_command(app: &mut MyApp, frame: &mut eframe::Frame){
        if app.get_request_state()==RequestState::Initialized {
            app.screen_request_init(frame);
        }else{
            app.reinit_app();
            app.screen_request_init(frame);
        }
    }
pub fn go_back_command(app: &mut MyApp){
    if app.is_rect_shown(){
        app.set_rect_shown(false);
        app.set_rect_position(1, Pos2::new(0.0,0.0));
        app.set_rect_position(2, Pos2::new(0.0,0.0));
    }else if app.get_request_state().equal("EditImage") {
        app.editing = None;
        app.erase_drawing();
        app.set_request_state(RequestState::Processed);
    }else if app.get_request_state().equal("HotkeyWindow"){
        app.set_request_state(RequestState::Initialized);
    }else if app.get_request_state().equal("HotkeysSelection"){
        app.clear_press_keys();
        app.set_request_state(RequestState::HotkeyWindow);

    }else if app.get_request_state().equal("HotkeysAdd"){
        app.set_request_state(RequestState::HotkeyWindow);

    } else{
        app.go_back();
    }
}

pub fn choice_monitor_command(app: &mut MyApp, frame: &mut eframe::Frame, number: usize){
    app.set_screen_selected(number-1);
    app.screen_request_init(frame);
}

pub fn save_image_command(app: &mut MyApp){
    //check if it's set default location and name
    let mut default_name : String = String::new();
    let mut location = String::new();
    if app.get_default().get_location().is_some(){
        location = app.get_default().get_location().unwrap();
    };
    if !app.get_default().get_name().is_empty(){
        default_name = format!("{} {}", app.get_default().get_name(), app.get_default().get_screenshot_numbers())
    }
    let path = FileDialog::new()
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG image",&["jpg", "jpeg"] )
        .add_filter("Gif image", &["gif"])
        .add_filter("WebP image", &["webp"])
        .add_filter("Tiff image", &["tiff"])
        .set_filename(default_name.as_str())
        .set_location(Path::new(location.as_str().clone()))
        .show_save_single_file().unwrap();
    //save image
    if let Some(path) = path {
        app.get_image_raw().save(path.as_path()).expect("Error in saving image");
    }
}
pub fn set_shortcut_command(app: &mut MyApp){
    app.set_hotkey_enable(app.get_hotkey_selected(), app.get_press_keys());
    app.clear_press_keys();
    app.set_repeated_keys(false);
    app.set_request_state(RequestState::HotkeyWindow);
}
pub fn select_screen_command(app: &mut MyApp, frame: &mut eframe::Frame){
    if app.is_rect_shown(){
        app.replace_image_with_rect();
        app.set_rect_choosen(true);
        let image_size = app.get_retained_image().size();
        frame.set_fullscreen(false);
        frame.set_maximized(false);
        let x_size = if (image_size[0] as f32)< 700.0{700.0}else{image_size[0] as f32};
        frame.set_window_size(Vec2::new(x_size, image_size[1] as f32*1.1+60.0));
        frame.set_centered();
    }
    app.set_request_state(RequestState::Processed); //transition to final state
    app.set_rect_shown(false);
}
pub fn reset_shortcut_command(app: &mut MyApp){
    app.clear_press_keys();
    app.set_repeated_keys(false);
}
pub fn delete_shortcut_command(app: &mut MyApp){
    app.clear_press_keys();
    app.remove_from_map_by_value(app.get_hotkey_selected());
    app.set_request_state(RequestState::HotkeyWindow);
}
pub fn ok_save_location_command(app: &mut MyApp){
    if app.image_raw.is_some() || app.is_rect_choosen() {
        app.set_request_state(RequestState::Processed)
    }else{
        app.set_request_state(RequestState::Initialized)
    }
}
pub fn draw_image_command(app: &mut MyApp, frame: &mut eframe::Frame, ui: &mut Ui) {
    if app.is_screen_made() {
        if app.get_request_state().equal("PROCESSED") || app.get_request_state().equal("EditImage"){
            ui.horizontal_centered(
                |ui| {
                    ui.vertical_centered(
                        |ui| {
                            app.ui_with_image(frame, ui);
                        });
                }
            );
        } else {
            app.ui_with_image(frame, ui);
        }
    }
}
}