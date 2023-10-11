pub mod app_enums{
    use std::collections::HashMap;
    use egui::Modifiers;
    use egui_extras::RetainedImage;
    use serde::{Deserialize, Serialize};

    ///enum for all screenshot type
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum ScreenshotType{
        FULL,
        CUSTOM
    }
    impl ScreenshotType{
        pub fn equal(self, state:&str) -> bool {
            match state{
                "FULL" =>{self ==ScreenshotType::FULL},
                "CUSTOM" =>{self ==ScreenshotType::CUSTOM },
                _ => {panic!("INVALID TYPE IN INPUT")}
            }
        }
    }
    ///enum for all request state
    #[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
    pub enum RequestState{
        Initialized, //non ho premuto il tasto +
        Incomplete, //caso in cui ho premuto il tasto + e devo fare lo screen
        ChoiceRect, //ho fatto lo screen per la fake trasparenza devo scegliere il rettangolo
        ChoiceMonitor, //caso di multi display sto alla schermata di scelta del monitor
        Processed, //ho terminato la richiesta
        HotkeyWindow,
        HotkeysAdd,
        HotkeysSelection, //scelgo le hotkeys
        EditImage,  //edit immagine
        SavePreferences
    }
    impl RequestState{
        pub fn equal(self, state:&str) -> bool {
            match state{
                "INITIALIZED" =>{self ==RequestState::Initialized },
                "INCOMPLETE" =>{self ==RequestState::Incomplete },
                "ChoiceRect" =>{self == RequestState::ChoiceRect },
                "ChoiceMonitor" =>{self == RequestState::ChoiceMonitor },
                "HotkeyWindow" =>{self == RequestState::HotkeyWindow },
                "HotkeysAdd" =>{self == RequestState::HotkeysAdd },
                "HotkeysSelection" =>{self == RequestState::HotkeysSelection },
                "EditImage" =>{self == RequestState::EditImage},
                "SavePreferences" =>{self == RequestState::SavePreferences}
                "PROCESSED" =>{self == RequestState::Processed },
                _ => {panic!("INVALID STATE IN INPUT")}
            }
        }
    }
    //enum for all hotkeys functions
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum HotkeysFunctions{
        NewFull,
        NewCustom
    }
    impl HotkeysFunctions{
        pub fn to_string(self) -> &'static str {
            match self {
                HotkeysFunctions::NewFull => {"FULL SCREEN"}
                HotkeysFunctions::NewCustom => {"CUSTOM SCREEN"}
                _ => {panic!("Impossible to transform in string")}
            }
        }
        pub fn into_enum(string: & str) -> Self{
            match string {
                "FULL SCREEN" =>{HotkeysFunctions::NewFull},
                "CUSTOM SCREEN" =>{HotkeysFunctions::NewCustom},
                _ => {panic!("INVALID Hotkeys functions IN INPUT")}
            }
        }
    }

    pub struct ImageToShow{
        pub(crate) full_ret_image: Option<RetainedImage>,
        pub(crate) custom_ret_image: Option<RetainedImage>,
        pub(crate) edit: Option<RetainedImage>
    }
    impl Default for ImageToShow{
        fn default() -> Self {
            Self{
                full_ret_image: None,
                custom_ret_image: None,
                edit: None
            }
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Serialize, Deserialize)]
    pub enum KeysEnum{
        Key(egui::Key),
        Modifier(Modifiers)
    }

    #[derive(Debug, PartialEq, Copy, Clone, Eq, Serialize, Deserialize)]
    pub enum RectEdit{
        HorizontalLeft,
        HorizontalRight,
        VerticalTop,
        VerticalDown
    }

    #[derive(Debug, PartialEq, Copy, Clone, Eq, Serialize, Deserialize)]
    pub enum EditType{
        Text,
        Painting,
    }
    impl EditType{
        pub fn equal(self, edit_type:&str) -> bool {
            match edit_type{
                "Text" =>{self == EditType::Text},
                "Painting" =>{self == EditType::Painting},
                _ => {panic!("INVALID EDIT TYPE IN INPUT")}
            }
        }
    }
    #[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
    pub struct SavedData{
        save_location: Option<String>,
        pub(crate)save_name: String,
        screenshot_numbers: usize,
    }
    impl Default for SavedData{
        fn default() -> Self {
            SavedData{save_location:None, save_name:"".into(), screenshot_numbers:0}
        }
    }
    impl SavedData{
        pub fn get_location(&self) ->Option<String>{self.save_location.clone()}
        pub fn get_name(&self) ->String{self.save_name.clone()}
        pub fn get_screenshot_numbers(&self) ->usize{self.screenshot_numbers}
        pub fn set_location(&mut self, location: String){self.save_location = Some(location)}
        pub fn set_name(&mut self, name: String){self.save_location = Some(name)}
        pub fn set_screenshot_number(&mut self, value: usize){self.screenshot_numbers = value}
    }
}