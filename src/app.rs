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
        use arboard::{Clipboard, ImageData};
        use cfg_if::cfg_if;
        use eframe;
        use egui;
        use egui::{Color32, Pos2, Ui};
        use egui_extras::RetainedImage;
        use screenshots::{DisplayInfo, Screen};
        use crate::app::screen_utils;
        use image;
        use image::{DynamicImage};
        use itertools::Itertools;
        use crate::enums::app_enums::{EditType, HotkeysFunctions, RectEdit, RequestState, DefaultOption, SavedData, SizeType};
        use crate::utils::utils::{retained_image_from_dynamic};

        pub struct MyApp {
            state: RequestState,
            pub(crate) image: Option<RetainedImage>,
            pub(crate) image_raw: Option<DynamicImage>,
            image_show:bool,
            pub(crate) hotkey_selected: HotkeysFunctions,
            hotkeys_enable: HashMap<Vec<String>, String>,
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
            pub(crate)highlight_size: Option<SizeType>,
            screen_made: bool,
            button_size: f32,
            font_size: f32,
            full_screen_request: bool
        }

        impl Default for MyApp{
            //default initialization of the app struct
            fn default() -> Self {
                Self{
                    state:RequestState::Initialized,
                    image:None,
                    image_raw:None,
                    image_show:false,
                    hotkey_selected: HotkeysFunctions::NewFull,
                    hotkeys_enable: HashMap::new(),
                    repeated_keys : false,
                    press_keys: vec![],
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
                    color: Color32::BLUE,
                    screen_made: false,
                    button_size: SIZE.clone(),
                    font_size: FONT_SIZE.clone(),
                    full_screen_request: false
                }
            }
        }
        ///METHODS OF MYAPP STRUCT
        impl MyApp{
            //--------------------------------------------------------------------------------------
            //GETTER
            pub fn get_request_state(&self) ->RequestState{ self.state }
            pub fn get_icon(&self, index:usize) -> &RetainedImage { self.icons.get(index).unwrap()}
            pub fn get_rect_position(&self) -> [Pos2;2]{ self.rect_positions }
            pub fn get_painting_position(&self) -> Vec<Vec<Pos2>>{self.painter_position.clone()}
            pub fn push_new_line(&mut self){self.painter_position.push(vec![])}
            pub fn push_new_position(&mut self, position:Pos2){
                let vec_size = self.painter_position.len().clone()-1;
                self.painter_position[vec_size].push(position)}
            pub fn get_image_raw(&mut self) -> &mut DynamicImage{self.image_raw.as_mut().unwrap()}
            pub fn get_retained_image(&mut self) -> &mut RetainedImage{self.image.as_mut().unwrap()}
            pub fn get_editing(&self) -> Option<EditType>{self.editing}
            pub fn get_highlight_size(&self) -> Option<SizeType>{self.highlight_size}
            pub fn get_rect_paint_position(&self) ->Vec<[Pos2;2]>{self.rect_paint_position.clone()}
            pub fn get_circle_paint_position(&self) ->Vec<[Pos2;2]>{self.circle_paint_position.clone()}
            pub fn get_arrow_paint_position(&self) ->Vec<[Pos2;2]>{self.arrow_paint_position.clone()}
            pub fn get_highlight_paint_position(&self) ->Vec<[Pos2;2]>{self.highlight_paint_position.clone()}
            pub fn get_hotkey_enable(&self) -> HashMap<Vec<String>, String>{self.hotkeys_enable.clone()}
            pub fn get_hotkey_selected(&self) -> HotkeysFunctions{self.hotkey_selected}
            pub fn get_default(&self) -> DefaultOption {self.saved_default.clone()}
            pub fn get_press_keys(&self) -> Vec<String>{self.press_keys.clone()}
            pub fn get_display_number(&self) -> usize{self.screen_number.clone()}
            pub fn get_delay(&self) -> i32 {self.delay.clone()}
            pub fn is_outside_rect(&self) -> bool{self.outside_rect.clone()}
            pub fn is_rect_choosen(&self) ->bool{self.rect_choosen.clone()}
            pub fn is_rect_shown(&self)->bool{ self.rect_shown.clone() }
            pub fn get_rect_edit(&self) -> Option<RectEdit>{self.rect_edit.clone()}
            pub fn is_repeated_keys(&self) -> bool{self.repeated_keys.clone()}
            pub fn is_setup(&self) ->bool{self.setup.clone()}
            pub fn is_first_processed(&self) -> bool{self.first_processed.clone()}
            pub fn get_screen_selected(&self) ->usize{self.screen_selected.clone()}
            pub fn is_edit_image(&self) -> bool{self.edit_image.clone()}
            pub fn is_screen_made(&self) ->bool{self.screen_made.clone()}
            pub fn is_image_show(&self) -> bool{self.image_show.clone()}
            pub fn get_size_button(&self) ->f32{self.button_size.clone()}
            pub fn get_font_size(&self) -> f32{self.font_size.clone()}
            pub fn is_full_screen_request(&self)->bool{self.full_screen_request.clone()}
            //--------------------------------------------------------------------------------------
            //SETTER
            pub fn set_request_state(&mut self, state: RequestState){
                self.state = state;
            }
            pub fn set_rect_position(&mut self, in_or_fin: usize, pos2:Pos2){
                if in_or_fin==1{self.rect_positions[0] = pos2}
                else {self.rect_positions[1] = pos2}
            }
            pub fn set_rect_choosen(&mut self, is_rect_choosen: bool){self.rect_choosen = is_rect_choosen}
            pub fn set_outside_rect(&mut self, is_outside_rect: bool){ self.outside_rect = is_outside_rect}
            pub fn set_screen_made(&mut self, value: bool){self.screen_made = value}
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
                    self.hotkeys_enable.insert(shortcuts[i.clone()].clone(), functions[i.clone()].clone());
                }
            }
            pub fn set_repeated_keys(&mut self, value: bool){self.repeated_keys = value}
            pub fn set_pressed_key(&mut self, keys: Vec<String>){self.press_keys = keys}
            pub fn set_rect_shown(&mut self, value: bool){self.rect_shown = value}
            pub fn set_rect_edit(&mut self, value: Option<RectEdit>){self.rect_edit = value}
            pub fn set_screen_selected(&mut self, value: usize){self.screen_selected = value}
            pub fn set_save_location(&mut self, location: String){self.saved_default.set_location(location)}
            pub fn set_setup(&mut self, value: bool){self.setup = value}
            pub fn set_first_processed(&mut self, value: bool){self.first_processed = value}
            pub fn set_full_screen_request(&mut self, value: bool){self.full_screen_request = value}
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
                self.image = Some(retained_image_from_dynamic(self.get_image_raw()).unwrap())
            }

            ///initizialized params for taking screen
            pub fn screen_request_init(&mut self, frame: &mut eframe::Frame){
                let displays = screen_utils::get_screen_number();
                self.is_multi_display = displays>1;
                self.screen_number = displays;
                self.increment_screenshot_number();
                //adjust frame to take screenshot easily
                if !self.is_multi_display.clone() || (self.is_multi_display.clone() && self.state.equal("ChoiceMonitor")){
                    frame.set_visible(false);
                    self.set_request_state(RequestState::Reframe);
                }else if self.is_multi_display.clone() && !self.state.equal("ChoiceMonitor"){
                    self.state = RequestState::ChoiceMonitor
                }
            }

            ///show image on ui based on frame window size
            /// @ frame and ui -> taken by egui implementation in main.rs
            pub fn load_image_on_screen(&mut self, ui: &mut Ui, frame: &mut eframe::Frame){
                //cerchiamo di mostrare l'immagine riempitiva un 80% del frame se non proviene da un edit
                if self.is_edit_image(){
                    self.get_retained_image().show(ui);
                }else{
                    if frame.info().window_info.size.x != 0.0 && frame.info().window_info.size.y != 0.0{
                        let frame_info = frame.info().window_info.size;
                        let x_image = frame_info.x*0.8;
                        let y_image = frame_info.y * 0.8;
                        if self.rect_choosen.clone() || self.get_request_state().equal("PROCESSED"){
                            let x_scale = x_image/self.image.as_ref().unwrap().width() as f32;
                            let y_scale = y_image/self.image.as_ref().unwrap().height() as f32;
                            let zoom = if x_scale <= y_scale { x_scale } else { y_scale };
                            self.get_retained_image().show_scaled(ui, zoom);
                        }else{
                            if !self.get_request_state().equal("PROCESSED"){
                                self.get_retained_image().show_scaled(ui, 0.92);
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
                self.load_image_on_screen(ui, frame);
                if self.is_first_processed() && (self.get_request_state().equal("PROCESSED") || self.rect_choosen.clone()) {
                    self.image_show=true;
                    //frame.set_centered();
                    self.set_first_processed(false);
                }
                //if rect type and not rect choosen then maximize the frame
                else if !self.is_rect_choosen() && !self.get_request_state().equal("PROCESSED"){
                    self.image_show=true;
                    frame.set_maximized(true);
                }
            }

            ///state-change from incomplete based on use case
            ///take the screenshot in case of full or rect mode of single display
            /// going in choice monitor if multi display usage
            pub fn process_screen_request(&mut self){
                self.rect_shown = false;
                if !self.is_multi_display.clone() || (self.is_multi_display.clone() && self.state.equal("Reframe")) {
                    self.set_request_state(RequestState::ChoiceScreen);
                }
            }

            /// method that crop full image to custom selected one
            pub fn replace_image_with_rect(&mut self){
                self.normalize_coords(screen_utils::get_screen(0).display_info);
                let x_starting_point = min(self.rect_positions[0].clone().x as i32, self.rect_positions[1].clone().x as i32);
                let y_starting_point = min(self.rect_positions[0].clone().y as i32, self.rect_positions[1].clone().y as i32);
                self.image_raw =Some(self.image_raw.as_mut().unwrap().crop_imm(
                    (x_starting_point) as u32,
                    (y_starting_point) as u32,
                    ((self.rect_positions[1].clone().x - self.rect_positions[0].clone().x).abs()) as u32,
                    ((self.rect_positions[1].clone().y - self.rect_positions[0].clone().y).abs()) as u32
                ));
                self.image=retained_image_from_dynamic(self.image_raw.as_ref().unwrap());
            }

            ///method to transform mouse coords inside frame in actual full image coords
            pub fn normalize_coords(&mut self, disp_info:DisplayInfo){
                let x_aspect_ratio: f32= disp_info.width as f32/((disp_info.width.clone()as f32*0.92)+12.0);
                let y_aspect_ratio:f32 = disp_info.height as f32 /((disp_info.height.clone() as f32 * 0.92)+12.0);
                //starting x point normalizing
                if self.rect_positions[0].x<12.0 { self.rect_positions[0].x= 0.0;}
                else{ self.rect_positions[0].x = self.rect_positions[0].clone().x*x_aspect_ratio.clone()}
                //starting y point normalizing
                if self.rect_positions[0].y<12.0{ self.rect_positions[0].clone().y=0.0;}
                else{self.rect_positions[0].y = self.rect_positions[0].clone().y*y_aspect_ratio.clone();}
                //normalizing final coords
                self.rect_positions[1].x=self.rect_positions[1].clone().x*x_aspect_ratio;
                self.rect_positions[1].y=self.rect_positions[1].clone().y*y_aspect_ratio;
            }

            ///take control of the go back flow for the frontend
            pub fn go_back(&mut self){
                    if !self.get_request_state().equal("EditImage"){
                        self.reinit_app();
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
                            self.reinit_app();
                            self.full_screen_request = true;
                            self.screen_request_init(frame);
                        }
                        HotkeysFunctions::NewCustom => {
                            self.reinit_app();
                            self.set_rect_position(1,Pos2::new(0.0,0.0));
                            self.set_rect_position(2,Pos2::new(0.0,0.0));
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
                self.image=None;
                self.image_raw= None;
                self.is_multi_display =false;
                self.screen_number=0;
                self.rect_positions=[Pos2::new(0.0,0.0), Pos2::new(0.0,0.0)];
                self.outside_rect=false;
                self.rect_choosen=false;
                self.edit_image = false;
                self.first_processed = false;
                self.editing = None;
                self.screen_made = false;
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
        cfg_if!{
            if #[cfg(target_os = "macos")]{
                static SIZE: f32 = 60.0;
                static FONT_SIZE : f32 = 28.0;
            }else{
                static SIZE: f32 = 30.0;
                static FONT_SIZE: f32 = 16.0;
            }
        }
    }

