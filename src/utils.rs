pub mod utils{
    use std::cmp::{max, min};
    use std::collections::HashMap;
    use egui::{Color32, ColorImage, CursorIcon, epaint, FontFamily, FontId, Modifiers, Pos2};
    use egui::FontFamily::Proportional;
    use egui::TextStyle::{Body, Button, Heading, Monospace, Small};
    use egui_extras::RetainedImage;
    use image::{DynamicImage};
    use itertools::Itertools;
    use crate::enums::app_enums::{HotkeysFunctions, RectEdit};
    use crate::app::app_utils::MyApp;

    ///retained image from  dynamic image
    pub fn retained_image_from_dynamic(dyn_image:&DynamicImage) -> Option<RetainedImage> {
        Some(RetainedImage::from_color_image(
            "screen", ColorImage::from_rgba_unmultiplied(
                [dyn_image.width() as _, dyn_image.height() as _],
                dyn_image.as_bytes()
            )))
    }

    ///compare function to sort key and modifiers

    ///method to find modifier given a list
    pub fn find_modifier(modifiers: &Modifiers) ->Option<Vec<&str>>{
        let mut result = vec![];
        if modifiers.matches(Modifiers::ALT){
            result.push("ALT");
        }else if modifiers.matches(Modifiers::CTRL){
            result.push("CTRL");
        }else if modifiers.matches(Modifiers::COMMAND){
            result.push("COMMAND");
        }else if modifiers.matches(Modifiers::MAC_CMD){
            result.push("MAC_CMD");
        }else if modifiers.matches(Modifiers::SHIFT){
            result.push("SHIFT");
        }else if modifiers.matches(Modifiers::CTRL | Modifiers::SHIFT) {
            result.push("CTRL");
            result.push("SHIFT");
        }else if modifiers.matches(Modifiers::CTRL | Modifiers::ALT){
            result.push("CTRL");
            result.push("ALT");
        }else if modifiers.matches(Modifiers::ALT | Modifiers::SHIFT){
            result.push("SHIFT");
            result.push("ALT");
        }else if modifiers.matches(Modifiers::CTRL | Modifiers::ALT|Modifiers::SHIFT){
            result.push("CTRL");
            result.push("ALT");
            result.push("SHIFT");
        }
        if !result.is_empty(){
            Some(result)
        }else{
            None
        }
    }

    ///method to stringify keys or modifier
    pub fn keys_string(keys: Vec<String>) -> String{
        keys.iter().join(" + ")
    }

    pub fn compare_keys(keys1 : Vec<String>, keys2: Vec<String>) -> bool{

        return if keys1 == keys2{
            true
        } else {
            false
        }
    }

    ///method to know all selectable hotkeys functions
    pub fn get_possible_hotkeys_functions(enable_functions: HashMap<Vec<String>, String>) -> Vec<HotkeysFunctions>{
        let mut all_functions = vec![HotkeysFunctions::NewFull, HotkeysFunctions::NewCustom];
        all_functions.retain(
            |function| !enable_functions.values().map(|v| HotkeysFunctions::into_enum(v.as_str())).contains(function)
        );
        all_functions
    }
    pub fn set_cursor(app : &MyApp, ctx: &egui::Context){
        if let Some(edit) = app.get_rect_edit(){
            if edit == RectEdit::HorizontalLeft || edit == RectEdit::HorizontalRight{
                ctx.set_cursor_icon(CursorIcon::ResizeHorizontal)
            }else{
                ctx.set_cursor_icon(CursorIcon::ResizeVertical)
            }
        }else{
            ctx.set_cursor_icon(CursorIcon::Default);
        }
    }

    pub fn change_rect(app : &mut MyApp, case: i32, position_to_update: f32){
        match case {
            1 =>{
                if app.get_rect_position()[0].x as i32 == min(app.get_rect_position()[0].x as i32, app.get_rect_position()[1].x as i32){
                    app.set_rect_position(1, Pos2::new(position_to_update, app.get_rect_position()[0].y))
                }else{
                    app.set_rect_position(2, Pos2::new(position_to_update, app.get_rect_position()[1].y))
                }
            }
            2 =>{
                if app.get_rect_position()[0].x as i32 == max(app.get_rect_position()[0].x as i32, app.get_rect_position()[1].x as i32){
                app.set_rect_position(1, Pos2::new(position_to_update, app.get_rect_position()[0].y))
            }else{
                app.set_rect_position(2, Pos2::new(position_to_update, app.get_rect_position()[1].y))
            }}
            3 => {
                if app.get_rect_position()[0].y as i32 == max(app.get_rect_position()[0].y as i32, app.get_rect_position()[1].y as i32) {
                    app.set_rect_position(1, Pos2::new(app.get_rect_position()[0].x, position_to_update))
                } else {
                    app.set_rect_position(2, Pos2::new(app.get_rect_position()[1].x, position_to_update))
                }
            }
            4 =>{
                if app.get_rect_position()[0].y as i32 == min(app.get_rect_position()[0].y as i32, app.get_rect_position()[1].y as i32){
                    app.set_rect_position(1, Pos2::new(app.get_rect_position()[0].x, position_to_update))
                }else{
                    app.set_rect_position(2, Pos2::new(app.get_rect_position()[1].x, position_to_update))
                }
            }
            _ => {}
        }
    }

    pub fn get_color_from_str(color: &str) -> Color32{
        match color {
            "RED" => Color32::RED,
            "BLUE" => Color32::BLUE,
            "GREEN" => Color32::GREEN,
            "YELLOW" => Color32::YELLOW,
            "BLACK" => Color32::BLACK,
            "BROWN" => Color32::BROWN,
            _ => Color32::BLACK
        }
    }

    pub fn get_str_from_color(color: Color32) -> String{
        match color {
            Color32::RED => "RED".to_string(),
            Color32::BLUE => "BLUE".to_string(),
            Color32::YELLOW => "YELLOW".to_string(),
            Color32::BROWN => "BROWN".to_string(),
            Color32::BLACK => "BLACK".to_string(),
            Color32::GREEN => "GREEN".to_string(),
            _ => "BLACK".to_string()
        }
    }
    pub fn custom_fonts(app: &MyApp, ctx: &egui::Context){
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(app.get_font_size(), Proportional)),
            (Body, FontId::new(app.get_font_size(), Proportional)),
            (Monospace, FontId::new(app.get_font_size(), Proportional)),
            (Button, FontId::new(app.get_font_size(), Proportional)),
            (Small, FontId::new(app.get_font_size() - 3.0, Proportional)),
        ]
            .into();
        ctx.set_style(style);
    }
}