pub mod app_enums{
    use egui::Modifiers;
    use egui_extras::RetainedImage;

    ///enum for all screenshot type
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum ScreenshotType{
        FULL,
        RECT
    }
    impl ScreenshotType{
        pub fn equal(self, state:&str) -> bool {
            match state{
                "FULL" =>{self ==ScreenshotType::FULL},
                "RECT" =>{self ==ScreenshotType::RECT},
                _ => {panic!("INVALID TYPE IN INPUT")}
            }
        }
    }
    ///enum for all request state
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum RequestState{
        INITIALIZED, //non ho premuto il tasto +
        INCOMPLETE, //caso in cui ho premuto il tasto + e devo fare lo screen
        CHOICE_RECT, //ho fatto lo screen per la fake trasparenza devo scegliere il rettangolo
        CHOICE_MONITOR, //caso di multi display sto alla schermata di scelta del monitor
        PROCESSED, //ho terminato la richiesta
        HOTKEY_WINDOW,
        HOTKEYS_ADD,
        HOTKEYS_SELECTION //scelgo le hotkeys
    }
    impl RequestState{
        pub fn equal(self, state:&str) -> bool {
            match state{
                "INITIALIZED" =>{self ==RequestState::INITIALIZED},
                "INCOMPLETE" =>{self ==RequestState::INCOMPLETE},
                "CHOICE_RECT" =>{self == RequestState::CHOICE_RECT},
                "CHOICE_MONITOR" =>{self == RequestState::CHOICE_MONITOR},
                "HOTKEY_WINDOW" =>{self == RequestState::HOTKEY_WINDOW},
                "HOTKEYS_ADD" =>{self == RequestState::HOTKEYS_ADD},
                "HOTKEYS_SELECTION" =>{self == RequestState::HOTKEYS_SELECTION},
                "PROCESSED" =>{self == RequestState::PROCESSED},
                _ => {panic!("INVALID STATE IN INPUT")}
            }
        }
    }
    //enum for all hotkeys functions
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum HotkeysFunctions{
        NewFull,
        NewCustom,
        QuarterTopRight,
        QuarterTopLeft,
        QuarterDownRight,
        QuarterDownLeft
    }
    impl HotkeysFunctions{
        pub fn equal(self, state:&str) -> bool {
            match state{
                "NewFull" =>{self ==HotkeysFunctions::NewFull},
                "NewCustom" =>{self ==HotkeysFunctions::NewCustom},
                "QuarterTopRight" =>{self == HotkeysFunctions::QuarterTopRight},
                "QuarterTopLeft" =>{self == HotkeysFunctions::QuarterTopLeft},
                "QuarterDownLeft" =>{self == HotkeysFunctions::QuarterDownLeft},
                "QuarterDownRight" =>{self == HotkeysFunctions::QuarterDownRight}
                _ => {panic!("INVALID Hotkeys functions IN INPUT")}
            }
        }
        pub fn to_string(self) -> &'static str {
            match self {
                HotkeysFunctions::NewFull => {"FULL SCREEN"}
                HotkeysFunctions::NewCustom => {"CUSTOM SCREEN"}
                HotkeysFunctions::QuarterTopRight => {"1/4 TOP L"}
                HotkeysFunctions::QuarterTopLeft => {"1/4 TOP R"}
                HotkeysFunctions::QuarterDownRight => {"1/4 DOWN R"}
                HotkeysFunctions::QuarterDownLeft => {"1/4 DOWN L"}
            }
        }
        pub fn into_enum(string: & str) -> Self{
            match string {
                "FULL SCREEN" =>{HotkeysFunctions::NewFull},
                "CUSTOM SCREEN" =>{HotkeysFunctions::NewCustom},
                "1/4 TOP R" =>{HotkeysFunctions::QuarterTopRight},
                "1/4 TOP L" =>{HotkeysFunctions::QuarterTopLeft},
                "1/4 DOWN R" =>{HotkeysFunctions::QuarterDownRight},
                "1/4 DOWN L" =>{HotkeysFunctions::QuarterDownLeft},
                _ => {panic!("INVALID Hotkeys functions IN INPUT")}
            }
        }
    }
    pub struct ImageToShow{
        pub(crate) full_ret_image: Option<RetainedImage>,
        pub(crate) custom_ret_image: Option<RetainedImage>
    }
    impl Default for ImageToShow{
        fn default() -> Self {
            Self{
                full_ret_image: None,
                custom_ret_image: None
            }
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
    pub enum KeysEnum{
        Key(egui::Key),
        Modifier(Modifiers)
    }
}