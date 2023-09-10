pub mod utils{
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use egui::{ColorImage, Modifiers};
    use egui_extras::RetainedImage;
    use image::DynamicImage;
    use itertools::Itertools;
    use crate::enums::app_enums::{HotkeysFunctions, KeysEnum, RequestState};
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
    pub(crate) fn sort_key_modifier(a: &KeysEnum, b: & KeysEnum) -> Ordering {
        if a == b {
            Ordering::Equal
        } else if *a == KeysEnum::Modifier(Modifiers::ALT) ||*a == KeysEnum::Modifier(Modifiers::CTRL) ||*a == KeysEnum::Modifier(Modifiers::SHIFT) || *a == KeysEnum::Modifier(Modifiers::COMMAND) || *a == KeysEnum::Modifier(Modifiers::MAC_CMD){
            Ordering::Less
        } else if (*a== KeysEnum::Modifier(Modifiers::CTRL) || *a==KeysEnum::Modifier(Modifiers::COMMAND)) && (*b == KeysEnum::Modifier(Modifiers::ALT) || *b == KeysEnum::Modifier(Modifiers::SHIFT)){
            Ordering::Less
        }else if (*a== KeysEnum::Modifier(Modifiers::SHIFT) ) && (*b == KeysEnum::Modifier(Modifiers::ALT)){
            Ordering::Less
        } else{
            Ordering::Greater
        }
    }

    ///method to find modifier given a list
    pub fn find_modifier(modifiers: &Modifiers) ->Option<Vec<Modifiers>>{
        let mut result = vec![];
        if modifiers.matches(Modifiers::ALT){
            result.push(Modifiers::ALT);
        }else if modifiers.matches(Modifiers::CTRL){
            result.push(Modifiers::CTRL);
        }else if modifiers.matches(Modifiers::COMMAND){
            result.push(Modifiers::COMMAND);
        }else if modifiers.matches(Modifiers::MAC_CMD){
            result.push(Modifiers::MAC_CMD);
        }else if modifiers.matches(Modifiers::SHIFT){
            result.push(Modifiers::SHIFT);
        }else if modifiers.matches(Modifiers::CTRL | Modifiers::SHIFT) {
            result.push(Modifiers::CTRL);
            result.push(Modifiers::SHIFT);
        }else if modifiers.matches(Modifiers::CTRL | Modifiers::ALT){
            result.push(Modifiers::CTRL);
            result.push(Modifiers::ALT);
        }else if modifiers.matches(Modifiers::ALT | Modifiers::SHIFT){
            result.push(Modifiers::SHIFT);
            result.push(Modifiers::ALT);
        }else if modifiers.matches(Modifiers::CTRL | Modifiers::ALT|Modifiers::SHIFT){
            result.push(Modifiers::CTRL);
            result.push(Modifiers::SHIFT);
            result.push(Modifiers::ALT);
        }
        if !result.is_empty(){
            Some(result)
        }else{
            None
        }
    }

    ///method to set keys or pressed keys
    pub fn set_keys_or_press_keys(app: &mut MyApp, state: RequestState, key: KeysEnum){
        if state.equal("HOTKEYS_SELECTION"){
            app.set_key(key);
        }else{
            app.set_pressed_key(key);
        }
    }

    ///method to stringify modifier
    pub fn stringify_mod(modifiers: &Modifiers) -> &'static str {
        if modifiers.matches(Modifiers::ALT){
            "ALT"
        }else if modifiers.matches(Modifiers::CTRL){
            "CTRL"
        }else if modifiers.matches(Modifiers::COMMAND){
            "COMMAND"
        }else if modifiers.matches(Modifiers::MAC_CMD){
            "MAC CMD"
        }else if modifiers.matches(Modifiers::SHIFT ){
            "SHIFT"
        }else{
            ""
        }
    }

    ///method to stringify keys or modifier
    pub fn keys_string(keys: Vec<KeysEnum>) -> String{
        keys.iter().map(
            |k|{
                match k{
                    KeysEnum::Key(key)=>{key.symbol_or_name()},
                    KeysEnum::Modifier(modifier)=>{stringify_mod(modifier)}
                }
            }
        ).unique().collect::<Vec<_>>().join(" + ")
    }

    ///method to sort pressed keys or hotkeys for normalizing equals
    pub fn sort_keys(vec: Vec<KeysEnum>) -> Vec<KeysEnum>{
        let mut sorted_vec = vec;
        sorted_vec.sort_by(sort_key_modifier);
        sorted_vec
    }

    ///method to know all selectable hotkeys functions
    pub fn get_possible_hotkeys_functions(enable_functions: HashMap<Vec<KeysEnum>, String>) -> Vec<HotkeysFunctions>{
        let mut all_functions = vec![HotkeysFunctions::NewFull, HotkeysFunctions::NewCustom, HotkeysFunctions::QuarterTopLeft, HotkeysFunctions::QuarterTopRight, HotkeysFunctions::QuarterDownLeft, HotkeysFunctions::QuarterDownRight];
        all_functions.retain(
            |function| !enable_functions.values().map(|v| HotkeysFunctions::into_enum(v.as_str())).contains(function)
        );
        all_functions
    }
}