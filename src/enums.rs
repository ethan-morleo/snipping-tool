pub mod app_enums{
    use egui::Modifiers;
    use egui_extras::RetainedImage;
    use std::collections::HashMap;
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
    pub enum SizeType{
        Small,
        Medium,
        Large
    }
    impl SizeType {
        pub fn equal(self, edit_type: &str) -> bool {
            match edit_type {
                "Small" => { self == SizeType::Small},
                "Medium" => { self == SizeType::Medium },
                "Large" => { self == SizeType::Large },
                _ => { panic!("INVALID EDIT TYPE IN INPUT") }
            }
        }
    }

    #[derive(Debug, PartialEq, Copy, Clone, Eq, Serialize, Deserialize)]
    pub enum EditType{
        Text,
        Free,
        Square,
        Circle,
        Arrow,
        Highlight
    }
    impl EditType{
        pub fn equal(self, edit_type:&str) -> bool {
            match edit_type{
                "Text" =>{self == EditType::Text},
                "Free" =>{self == EditType::Free},
                "Square" =>{self == EditType::Square},
                "Arrow" => {self == EditType::Arrow}
                "Highlight" =>{self == EditType::Highlight}
                _ => {panic!("INVALID EDIT TYPE IN INPUT")}
            }
        }
        pub fn to_string(self) -> String{
            match self {
                EditType::Text => {"".to_string()}
                EditType::Free => {"🖊 Free".to_string()}
                EditType::Square => {"Square".to_string()}
                EditType::Circle => {"Circle".to_string()}
                EditType::Arrow => {"Arrow".to_string()}
                EditType::Highlight =>{"Highlight".to_string()}
            }
        }
        pub fn from_string(value: &str) -> Self{
            match value{
                "🖊 Free" =>{EditType::Free}
                "Square" => {EditType::Square}
                "Circle" => {EditType::Circle}
                "Arrow" =>{EditType::Arrow}
                "Highlight" =>{EditType::Highlight}
                _ => {panic!("INVALID EDIT TYPE IN INPUT")}
            }
        }
    }
    #[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
    pub struct DefaultOption {
        save_location: Option<String>,
        pub(crate)save_name: String,
        screenshot_numbers: usize,
    }
    impl Default for DefaultOption {
        fn default() -> Self {
            DefaultOption {save_location:None, save_name:"".into(), screenshot_numbers:0}
        }
    }
    impl DefaultOption {
        pub fn get_location(&self) ->Option<String>{self.save_location.clone()}
        pub fn get_name(&self) ->String{self.save_name.clone()}
        pub fn get_screenshot_numbers(&self) ->usize{self.screenshot_numbers}
        pub fn set_location(&mut self, location: String){self.save_location = Some(location)}
        pub fn set_screenshot_number(&mut self, value: usize){self.screenshot_numbers = value}
    }

    #[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
    pub struct SavedData{
        hotkeys_function: Vec<String>,
        shortcuts: Vec<Vec<String>>,
        default: DefaultOption
    }
    impl SavedData{

        pub fn new(hotkeys_function: Vec<String>, shortcuts: Vec<Vec<String>>, default: DefaultOption) -> SavedData{
            SavedData{hotkeys_function, shortcuts, default }
        }
        pub fn get_hotkeys_function(&self) -> Vec<String>{self.hotkeys_function.clone()}
        pub fn get_shortcuts(&self) -> Vec<Vec<String>>{self.shortcuts.clone()}
        pub fn get_default(&self) -> DefaultOption{self.default.clone()}

    }
    impl Default for SavedData{
        fn default() -> Self {
            Self{
                hotkeys_function: vec![],
                shortcuts: vec![],
                default: DefaultOption::default()
            }
        }
    }
}