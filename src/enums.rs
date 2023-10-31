pub mod app_enums{
    use egui::Modifiers;
    use egui_extras::RetainedImage;
    use serde::{Deserialize, Serialize};

    ///enum for all request state
    #[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
    pub enum RequestState{
        Initialized, //home page
        Reframe, //between press + button and taking screen
        ChoiceScreen, //after taking full screen, choosing the custom area
        ChoiceMonitor, //if multidisplay, monitor selection page
        Processed, //terminal state
        HotkeyWindow,
        HotkeysAdd,
        HotkeysSelection, //hotkeys states
        EditImage,  //painting on image page
        SavePreferences //save default location and name page
    }
    impl RequestState{
        pub fn equal(self, state:&str) -> bool {
            match state{
                "INITIALIZED" =>{self ==RequestState::Initialized },
                "Reframe" =>{self ==RequestState::Reframe },
                "ChoiceScreen" =>{self == RequestState::ChoiceScreen },
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