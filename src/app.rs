    pub(crate) mod screen_utils{
        use image::{DynamicImage, RgbaImage};
        use screenshots::{Screen};

        pub fn get_screen_number() -> usize{ Screen::all().unwrap().len() }

        pub fn get_screen(number:usize) -> Screen{ Screen::all().unwrap().get(number).expect("No screen retrieved").clone() }


        pub fn take_full_screenshot(screen: Screen) -> DynamicImage {
            match screen.capture(){
                Ok(image) => {
                    let rgba_image= RgbaImage::from_raw(
                        image.width() as u32,
                        image.height() as u32,
                        image.rgba().clone()
                    ).unwrap();
                    DynamicImage::from(rgba_image)
                }
                Err(e) => {panic!("error: {}", e)}
            }

        }

    }
    pub(crate) mod app_utils{
        use std::borrow::Cow;
        use std::cmp::{min};
        use std::collections::HashMap;
        use std::thread;
        use std::time::Duration;
        use arboard::{Clipboard, ImageData};
        use eframe;
        use egui;
        use egui::{Color32, Pos2, Ui};
        use egui_extras::RetainedImage;
        use screenshots::{DisplayInfo, Screen};
        use crate::app::screen_utils;
        use image;
        use image::{DynamicImage};
        use itertools::Itertools;
        use crate::app::screen_utils::get_screen;
        use crate::enums::app_enums::{EditType, HotkeysFunctions, ImageToShow, RectEdit, RequestState, DefaultOption, ScreenshotType, SavedData, SizeType};
        use crate::utils::utils::{retained_image_from_dynamic};

        pub struct MyApp {
            state: RequestState,
            image: ImageToShow,
            pub(crate) image_raw: Option<DynamicImage>,
            image_show:bool,
            erased:bool,
            pub(crate) hotkey_selected: HotkeysFunctions,
            hotkeys_enable: HashMap<Vec<String>, String>,
            pub(crate) screen_type: ScreenshotType,
            repeated_keys : bool,
            press_keys: Vec<String>,
            is_multi_display: bool,
            screen_number: usize,
            screen_selected: usize,
            rect_positions: [Pos2;2],
            rect_edit: Option<RectEdit>,
            outside_rect: bool,
            rect_shown: bool,
            rect_choosen: bool,
            icons: Vec<RetainedImage>,
            pub(crate) saved_default: DefaultOption,
            setup: bool,
            pub(crate)delay: i32,
            painter_position: Vec<Vec<Pos2>>,
            rect_paint_position: Vec<[Pos2;2]>,
            circle_paint_position: Vec<[Pos2;2]>,
            arrow_paint_position: Vec<[Pos2; 2]>,
            highlight_paint_position: Vec<[Pos2;2]>,
            pub(crate)editing: Option<EditType>,
            first_processed: bool,
            edit_image: bool,
            pub(crate)color: Color32,
            pub(crate)highlight_size: Option<SizeType>
        }

        impl Default for MyApp{
            //default initialization of the app struct
            fn default() -> Self {
                Self{
                    state:RequestState::Initialized,
                    image:ImageToShow::default(),
                    image_raw:None,
                    image_show:false,
                    erased:false,
                    hotkey_selected: HotkeysFunctions::NewFull,
                    hotkeys_enable: HashMap::new(),
                    repeated_keys : false,
                    press_keys: vec![],
                    screen_type:ScreenshotType::FULL,
                    is_multi_display:false,
                    screen_number:0,
                    screen_selected: 0,
                    rect_positions:[Pos2::new(0.0,0.0), Pos2::new(0.0,0.0)],
                    rect_edit : None,
                    rect_shown: false,
                    outside_rect:false,
                    rect_choosen:false,
                    icons:init_icons(),
                    saved_default: DefaultOption::default(),
                    setup: false,
                    delay: 0,
                    painter_position: vec![],
                    rect_paint_position: vec![],
                    circle_paint_position: vec![],
                    arrow_paint_position: vec![],
                    highlight_paint_position: vec![],
                    editing: None,
                    highlight_size: Some(SizeType::Small),
                    first_processed: false,
                    edit_image: false,
                    color: Color32::BLUE
                }
            }
        }
        ///METHODS OF MYAPP STRUCT
        impl MyApp{
            //--------------------------------------------------------------------------------------
            //GETTER
            pub fn get_request_state(&self) ->RequestState{ self.state }
            pub fn get_screen_type(&self) -> ScreenshotType{ self.screen_type }
            pub fn get_icon(&self, index:usize) -> &RetainedImage { self.icons.get(index).unwrap()}
            pub fn get_rect_position(&self) -> [Pos2;2]{ self.rect_positions }

            pub fn get_painting_position(&self) -> Vec<Vec<Pos2>>{self.painter_position.clone()}
            pub fn push_new_line(&mut self){self.painter_position.push(vec![])}
            pub fn push_new_position(&mut self, position:Pos2){
                let size = self.painter_position.len().clone()-1;
                self.painter_position[size].push(position)}
            pub fn get_image_raw(&mut self) -> &mut DynamicImage{self.image_raw.as_mut().unwrap()}
            pub fn get_full_image(&mut self) -> &mut RetainedImage{self.image.full_ret_image.as_mut().unwrap()}
            pub fn get_editing(&self) -> Option<EditType>{self.editing}
            pub fn get_highlight_size(&self) -> Option<SizeType>{self.highlight_size}
            pub fn get_rect_paint_position(&self) ->Vec<[Pos2;2]>{self.rect_paint_position.clone()}
            pub fn get_circle_paint_position(&self) ->Vec<[Pos2;2]>{self.circle_paint_position.clone()}
            pub fn get_arrow_paint_position(&self) ->Vec<[Pos2;2]>{self.arrow_paint_position.clone()}
            pub fn get_highlight_paint_position(&self) ->Vec<[Pos2;2]>{self.highlight_paint_position.clone()}
            pub fn get_rect_image(&mut self) -> &mut RetainedImage{self.image.custom_ret_image.as_mut().unwrap()}
            pub fn get_hotkey_enable(&self) -> HashMap<Vec<String>, String>{self.hotkeys_enable.clone()}
            pub fn get_hotkey_selected(&self) -> HotkeysFunctions{self.hotkey_selected}
            pub fn get_default(&self) -> DefaultOption {self.saved_default.clone()}
            pub fn get_press_keys(&self) -> Vec<String>{self.press_keys.clone()}
            pub fn get_display_number(&self) -> usize{self.screen_number}
            pub fn get_delay(&self) -> i32 {self.delay}
            pub fn is_erased(&self) -> bool{
                self.erased
            }
            pub fn is_outside_rect(&self) -> bool{self.outside_rect}
            pub fn is_rect_choosen(&self) ->bool{self.rect_choosen}
            pub fn is_rect_shown(&self)->bool{ self.rect_shown }
            pub fn get_rect_edit(&self) -> Option<RectEdit>{self.rect_edit.clone()}
            pub fn is_repeated_keys(&self) -> bool{self.repeated_keys}
            pub fn is_setup(&self) ->bool{self.setup}
            pub fn is_first_processed(&self) -> bool{self.first_processed}

            pub fn is_edit_image(&self) -> bool{self.edit_image}
            //--------------------------------------------------------------------------------------
            //SETTER
            pub fn set_request_state(&mut self, state: RequestState){
                self.state = state;
            }
            pub fn set_erased(&mut self, is_erased: bool) {
                self.erased = is_erased
            }
            pub fn set_screen_type(&mut self, screen_type: ScreenshotType){ self.screen_type = screen_type }
            pub fn set_rect_position(&mut self, in_or_fin: usize, pos2:Pos2){
                if in_or_fin==1{self.rect_positions[0] = pos2}
                else {self.rect_positions[1] = pos2}
            }
            pub fn set_rect_choosen(&mut self, is_rect_choosen: bool){self.rect_choosen = is_rect_choosen}
            pub fn set_outside_rect(&mut self, is_outside_rect: bool){ self.outside_rect = is_outside_rect}
            pub fn set_hotkey_selected(&mut self, function: HotkeysFunctions){self.hotkey_selected = function}
            pub fn set_hotkey_enable(&mut self, function: HotkeysFunctions, keys: Vec<String>){
                //check on keys (if hotkeys already exist)
                if self.hotkeys_enable.contains_key(&*keys){
                    *self.hotkeys_enable.get_mut(&*keys).unwrap() = function.to_string().parse().unwrap()
                }
                //check on values (hotkeys function if already exist)
                if !self.hotkeys_enable.values().contains(&function.to_string().to_string()){
                    self.hotkeys_enable.insert(keys, function.to_string().to_string());
                }else{
                    //rimuovo l'elemento cercando la key associata al valore
                    self.remove_from_map_by_value( function);
                    //inserisco dopo aver eliminato
                    self.hotkeys_enable.insert(keys, function.to_string().to_string());
                }
            }
            pub fn set_hotkeys_map(&mut self, functions: Vec<String>, shortcuts: Vec<Vec<String>>){
                self.hotkeys_enable = HashMap::new();
                for i in 0..functions.len(){
                    self.hotkeys_enable.insert(shortcuts[i].clone(), functions[i].clone());
                }
            }
            pub fn set_repeated_keys(&mut self, value: bool){self.repeated_keys = value}
            pub fn set_pressed_key(&mut self, key: &str){self.press_keys.push(key.to_string())}
            pub fn set_rect_shown(&mut self, value: bool){self.rect_shown = value}
            pub fn set_rect_edit(&mut self, value: Option<RectEdit>){self.rect_edit = value}
            pub fn set_screen_selected(&mut self, value: usize){self.screen_selected = value}
            pub fn set_image_to_show(&mut self, image: RetainedImage){
                if self.screen_type.equal("FULL"){
                    self.image.full_ret_image = Some(image);
                }else{
                    self.image.custom_ret_image = Some(image);
                }
            }
            pub fn set_save_location(&mut self, location: String){self.saved_default.set_location(location)}
            pub fn set_setup(&mut self, value: bool){self.setup = value}
            pub fn set_first_processed(&mut self, value: bool){self.first_processed = value}
            pub fn set_saved_default(&mut self, data: DefaultOption){self.saved_default = data}
            pub fn set_edit_image(&mut self, value: bool){self.edit_image = value}
            pub fn set_new_rect_position(&mut self, pos: [Pos2; 2]){ self.rect_paint_position.push(pos)}
            pub fn update_rect_position(&mut self, pos: Pos2){
                let len = self.rect_paint_position.len().clone() -1;
                self.rect_paint_position[len][1] = pos;
            }
            pub fn set_new_circle_position(&mut self, pos: [Pos2; 2]){ self.circle_paint_position.push(pos)}
            pub fn update_circle_position(&mut self, pos: Pos2){
                let len = self.circle_paint_position.len().clone() -1;
                self.circle_paint_position[len][1] = pos;
            }
            pub fn set_new_arrow_position(&mut self, pos: [Pos2; 2]){ self.arrow_paint_position.push(pos)}
            pub fn update_arrow_position(&mut self, pos: Pos2){
                let len = self.arrow_paint_position.len().clone() -1;
                self.arrow_paint_position[len][1] = pos;
            }
            pub fn set_new_highlight_position(&mut self, pos: [Pos2; 2]){ self.highlight_paint_position.push(pos)}
            pub fn update_highlight_position(&mut self, pos: Pos2){
                let len = self.highlight_paint_position.len().clone() -1;
                self.highlight_paint_position[len][1] = pos;
            }
            //--------------------------------------------------------------------------------------
            //UTILS

            pub fn erase_image_to_show(&mut self){
                self.image.full_ret_image= None;
                self.image.custom_ret_image = None;
            }

            pub fn remove_from_map_by_value(&mut self, value: HotkeysFunctions){
                let remove_key = self.hotkeys_enable
                    .iter()
                    .find(|(_, &ref val)| val.eq_ignore_ascii_case(value.to_string()))
                    .map(|(key, _)| key.clone());

                if let Some(keys) = remove_key{
                    self.hotkeys_enable.remove(&*keys);
                }
            }

            pub fn increment_screenshot_number(&mut self){
                self.saved_default.set_screenshot_number(self.saved_default.get_screenshot_numbers()+1)
            }

            pub fn setup(&mut self){
                let restore_data: SavedData = confy::load("rust-snipping-tool", None).unwrap_or_default();
                let hotkeys_function = restore_data.get_hotkeys_function();
                let shortcuts = restore_data.get_shortcuts();
                let default = restore_data.get_default();
                self.set_hotkeys_map(hotkeys_function, shortcuts);
                self.set_saved_default(default);
            }

            pub fn erase_drawing(&mut self){
                self.painter_position.clear();
                self.rect_paint_position.clear();
                self.circle_paint_position.clear();
                self.arrow_paint_position.clear();
                self.highlight_paint_position.clear();
            }
            //--------------------------------------------------------------------------------------
            //PUBLIC METHOD

            ///take full display screenshot
            /// @screen: Display to screen -> type of Screenshots crate
            pub fn full_screenshot(&mut self, screen: &Screen){
                self.image_raw= Some(screen_utils::take_full_screenshot(*screen));
                self.image.full_ret_image =retained_image_from_dynamic(self.image_raw.as_ref().unwrap());
            }

            ///initizialized params for taking screen
            pub fn screen_request_init(&mut self, frame: &mut eframe::Frame){
                let displays = screen_utils::get_screen_number();
                self.is_multi_display = displays>1;
                self.screen_number = displays;
                self.increment_screenshot_number();
                //adjust frame to take screenshot easily
                if !self.is_multi_display || (self.is_multi_display && self.state.equal("ChoiceMonitor")){
                    self.state = RequestState::Incomplete;
                    if self.screen_type.equal("FULL"){
                        frame.set_minimized(true);
                    }else{
                        if!self.get_request_state().equal("PROCESSED"){
                            frame.set_minimized(true);
                        }
                        //frame.set_window_pos(Pos2::new(-5000.0,-5000.0));
                    }
                }else if self.is_multi_display && !self.state.equal("ChoiceMonitor"){
                    self.state = RequestState::ChoiceMonitor
                }
            }

            ///show image on ui based on frame window size
            /// @ frame and ui -> taken by egui implementation in main.rs
            pub fn show_image(&mut self, ui: &mut Ui, frame: &mut eframe::Frame){
                //cerchiamo di mostrare l'immagine riempitiva un 80% del frame se non proviene da un edit
                if self.is_edit_image(){
                    if self.get_screen_type().equal("FULL"){
                        self.get_full_image().show(ui);
                    }else{
                        self.get_rect_image().show(ui);
                    }
                }else{
                    if frame.info().window_info.size.x != 0.0 && frame.info().window_info.size.y != 0.0{
                        let frame_info = frame.info().window_info.size;
                        let x_image = frame_info.x*0.8;
                        let y_image = frame_info.y * 0.8;
                        if self.get_screen_type().equal("FULL"){
                            let x_scale = x_image/self.get_full_image().width() as f32;
                            let y_scale = y_image/self.get_full_image().height() as f32;
                            let zoom = if x_scale <= y_scale { x_scale } else { y_scale };
                            self.get_full_image().show_scaled(ui, zoom);
                        }else{
                            if self.is_rect_choosen(){
                                let x_scale = x_image/self.get_rect_image().width() as f32;
                                let y_scale = y_image/self.get_rect_image().height() as f32;
                                let zoom = if x_scale <= y_scale { x_scale } else { y_scale };
                                self.get_rect_image().show_scaled(ui, zoom as f32);
                            }else{
                                if !self.get_request_state().equal("PROCESSED"){
                                    self.image.full_ret_image.as_mut().unwrap().show_scaled(ui, 0.92);
                                }
                            }

                        }
                    }
                }

            }
            ///reformat frame and ui with image
            /// screen_type==FULL-> give attention to user to reopen frame
            /// screen_type==RECT -> maximize frame with the full image shown
            /// screen_type==RECT && rect_choosen -> almost 100%, resize frame
            /// @ frame and ui taken from main.rs
            pub fn ui_with_image(&mut self, frame: &mut eframe::Frame, ui: &mut Ui){
                self.show_image(ui, frame);
                if self.screen_type.equal("FULL") && self.is_first_processed() {
                    self.image_show=true;
                    frame.set_centered();
                    self.set_first_processed(false);
                }
                //if rect type and not rect choosen then maximize the frame
                else if self.screen_type==ScreenshotType::CUSTOM && !self.is_rect_choosen(){
                    self.image_show=true;
                    frame.set_minimized(false);
                    frame.set_always_on_top(true);
                    frame.focus();
                    frame.set_maximized(true);
                }
                //if rect type and rect choosen set maximized false and fullscreen false
                else if self.screen_type==ScreenshotType::CUSTOM && self.is_rect_choosen() && self.is_first_processed(){
                    self.image_show=true;
                    self.set_first_processed(false);
                }
            }

            ///state-change from incomplete based on use case
            ///take the screenshot in case of full or rect mode of single display
            /// going in choice monitor if multi display usage
            pub fn process_incomplete_request(&mut self){
                self.erased =false;
                self.rect_shown = false;
                if !self.is_multi_display || (self.is_multi_display && self.state.equal("INCOMPLETE")) {
                    //caso full
                    if self.get_delay() !=0{
                        thread::sleep(Duration::from_secs(self.get_delay() as u64))
                    }
                    if self.screen_type.equal("FULL") {
                        self.full_screenshot(&get_screen(self.screen_selected));
                        self.set_request_state(RequestState::Processed);
                    }
                    //caso rect
                    else {
                        self.full_screenshot(&get_screen(self.screen_selected));
                        self.set_request_state(RequestState::ChoiceRect);
                    }
                }
            }

            /// method that crop full image to custom selected one
            pub fn replace_image_with_rect(&mut self){
                self.normalize_coords(screen_utils::get_screen(0).display_info);
                let x_starting_point = min(self.rect_positions[0].x as i32, self.rect_positions[1].x as i32);
                let y_starting_point = min(self.rect_positions[0].y as i32, self.rect_positions[1].y as i32);
                self.image_raw =Some(self.image_raw.as_mut().unwrap().crop_imm(
                    (x_starting_point) as u32,
                    (y_starting_point) as u32,
                    ((self.rect_positions[1].x - self.rect_positions[0].x).abs()) as u32,
                    ((self.rect_positions[1].y - self.rect_positions[0].y).abs()) as u32
                ));
                self.image.custom_ret_image=retained_image_from_dynamic(self.image_raw.as_ref().unwrap());
            }

            ///method to transform mouse coords inside frame in actual full image coords
            pub fn normalize_coords(&mut self, disp_info:DisplayInfo){
                let x_aspect_ratio: f32= disp_info.width as f32/((disp_info.width as f32*0.92)+12.0);
                let y_aspect_ratio:f32 = disp_info.height as f32 /((disp_info.height as f32 * 0.92)+12.0);
                //starting x point normalizing
                if self.rect_positions[0].x<12.0 { self.rect_positions[0].x = 0.0;}
                else{ self.rect_positions[0].x = self.rect_positions[0].x*x_aspect_ratio}
                //starting y point normalizing
                if self.rect_positions[0].y<12.0{ self.rect_positions[0].y=0.0;}
                else{self.rect_positions[0].y = self.rect_positions[0].y*y_aspect_ratio;}
                //normalizing final coords
                self.rect_positions[1].x=self.rect_positions[1].x*x_aspect_ratio;
                self.rect_positions[1].y=self.rect_positions[1].y*y_aspect_ratio;
            }

            ///take control of the go back flow for the frontend
            pub fn go_back(&mut self){
                    if !self.get_request_state().equal("EditImage"){
                        self.reinit_app();
                        self.erased=true;
                    }else{
                        self.set_edit_image(false);
                        self.set_request_state(RequestState::Processed);
                    }

            }

            ///copy image in clipboard
            pub fn copy_in_clipboard(&mut self, clipboard: &mut Clipboard){

                let img_to_copy = self.image_raw.as_mut().unwrap();
                Clipboard::set_image(
                    clipboard,
                    ImageData {
                        width: img_to_copy.width() as usize,
                        height: img_to_copy.height() as usize,
                        bytes: Cow::from(img_to_copy.as_bytes())
                    }
                ).expect("impossible to copy in clipboard");
            }

            ///execute hotkey_function
            pub fn do_hotkey_function(&mut self, function: HotkeysFunctions, frame: &mut eframe::Frame){
                if self.state.equal("INITIALIZED") || self.state.equal("PROCESSED") || self.state.equal("HotkeyWindow"){
                    match function {
                        HotkeysFunctions::NewFull => {
                            self.set_screen_type(ScreenshotType::FULL);
                            self.set_request_state(RequestState::Initialized);
                            self.erase_image_to_show();
                            self.screen_request_init(frame);
                        }
                        HotkeysFunctions::NewCustom => {
                            self.set_screen_type(ScreenshotType::CUSTOM);
                            self.rect_choosen = false;
                            self.set_rect_position(1,Pos2::new(0.0,0.0));
                            self.set_rect_position(2,Pos2::new(0.0,0.0));
                            self.set_request_state(RequestState::Initialized);
                            self.erase_image_to_show();
                            self.screen_request_init(frame);
                        }
                    }
                }
            }
            pub fn clear_press_keys(&mut self){
                self.press_keys.clear();
            }

            //--------------------------------------------------------------------------------------
            //REINIT APP FOR HANDLING PREVIOUS STATES
            pub(crate) fn reinit_app(&mut self){
                self.state =RequestState::Initialized;
                self.image=ImageToShow::default();
                self.image_raw= None;
                self.erased=false;
                self.screen_type= ScreenshotType::FULL;
                self.is_multi_display =false;
                self.screen_number=0;
                self.rect_positions=[Pos2::new(0.0,0.0), Pos2::new(0.0,0.0)];
                self.outside_rect=false;
                self.rect_choosen=false;
                self.edit_image = false;
                self.first_processed = false;
                self.editing = None;
            }
        }
        //------------------------------------------------------------------------------------------
        //STATIC METHODS
        ///static method to initialized icons for ui
        fn init_icons() -> Vec<RetainedImage> {
            vec![
                RetainedImage::from_image_bytes(
                    "add icon", //0
                    include_bytes!("../icons/add.png"),
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "display icon", //1
                    include_bytes!("../icons/display.png"),
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "erase icon", //2
                    include_bytes!("../icons/eraser.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "ok icon", //3
                    include_bytes!("../icons/ok.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "back_icon", //4
                    include_bytes!("../icons/back.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "folder", //5
                    include_bytes!("../icons/folder.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "copy", //6
                    include_bytes!("../icons/copy.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "more", //7
                    include_bytes!("../icons/more.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "shortcut", //8
                    include_bytes!("../icons/shortcut.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "edit", //9
                    include_bytes!("../icons/edit.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "draw", //10
                    include_bytes!("../icons/draw.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "text", //11
                    include_bytes!("../icons/text.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "text", //12
                    include_bytes!("../icons/save_edit.png")
                ).unwrap()
            ]
        }
    }

