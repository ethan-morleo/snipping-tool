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
        use eframe;
        use egui;
        use egui::{Pos2, Ui, UserAttentionType, Vec2};
        use egui_extras::RetainedImage;
        use screenshots::{DisplayInfo, Screen};
        use crate::app::screen_utils;
        use image;
        use image::{DynamicImage};
        use native_dialog::FileDialog;
        use crate::enums::app_enums::{HotkeysFunctions, ImageToShow, KeysEnum, RequestState, ScreenshotType};
        use crate::utils::utils::retained_image_from_dynamic;


        pub struct MyApp {
            request_state: RequestState,
            image: ImageToShow,
            pub(crate) image_raw: Option<DynamicImage>,
            image_show:bool,
            erased:bool,
            hotkeys_enable: HashMap<Vec<KeysEnum>, String>,
            pub(crate) hotkey_selected: HotkeysFunctions,
            pub(crate) screen_type: ScreenshotType,
            keys: Vec<KeysEnum>,
            press_keys: Vec<KeysEnum>,
            is_multi_display: bool,
            screen_number: usize,
            rect_positions: [Pos2;2],
            outside_rect: bool,
            rect_choosen: bool,
            icons: Vec<RetainedImage>,
        }

        impl Default for MyApp{
            //default initialization of the app struct
            fn default() -> Self {
                Self{
                    request_state:RequestState::INITIALIZED,
                    image:ImageToShow::default(),
                    image_raw:None,
                    image_show:false,
                    erased:false,
                    hotkey_selected: HotkeysFunctions::NewFull,
                    hotkeys_enable: HashMap::new(),
                    keys: vec![],
                    press_keys: vec![],
                    screen_type:ScreenshotType::FULL,
                    is_multi_display:false,
                    screen_number:0,
                    rect_positions:[Pos2::new(0.0,0.0), Pos2::new(0.0,0.0)],
                    outside_rect:false,
                    rect_choosen:false,
                    icons:init_icons()
                }
            }
        }
        ///METHODS OF MYAPP STRUCT
        impl MyApp{
            //--------------------------------------------------------------------------------------
            //GETTER
            pub fn get_request_state(&self) ->RequestState{ self.request_state}
            pub fn get_screen_type(&self) -> ScreenshotType{ self.screen_type }
            pub fn get_icon(&self, index:usize) -> &RetainedImage { self.icons.get(index).unwrap()}
            pub fn get_rect_position(&self) -> [Pos2;2]{ self.rect_positions }
            pub fn get_image_raw(&mut self) -> &mut DynamicImage{self.image_raw.as_mut().unwrap()}
            pub fn get_full_image(&mut self) -> &mut RetainedImage{self.image.full_ret_image.as_mut().unwrap()}
            pub fn get_rect_image(&mut self) -> &mut RetainedImage{self.image.custom_ret_image.as_mut().unwrap()}
            pub fn get_hotkey_enable(&self) -> HashMap<Vec<KeysEnum>, String>{self.hotkeys_enable.clone()}
            pub fn get_hotkey_selected(&self) -> HotkeysFunctions{self.hotkey_selected}
            pub fn get_keys(&self) -> Vec<KeysEnum>{self.keys.clone()}
            pub fn get_press_keys(&self) -> Vec<KeysEnum>{self.press_keys.clone()}
            pub fn is_erased(&self) -> bool{
                self.erased
            }
            pub fn is_outside_rect(&self) -> bool{self.outside_rect}
            pub fn is_rect_choosen(&self) ->bool{self.rect_choosen}
            pub fn is_rect_shown(&self)->bool{
                //se vedo il rettangolo rosso i due elementi sono diversi tra loro e non sono entrambi uguali a 0
                self.rect_positions[0]!=Pos2::new(0.0,0.0) && self.rect_positions[1]!= Pos2::new(0.0,0.0)
            }

            //--------------------------------------------------------------------------------------
            //SETTER
            pub fn set_request_state(&mut self, state: RequestState){
                self.request_state = state;
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

            pub fn set_hotkey_enable(&mut self, function: HotkeysFunctions, keys: Vec<KeysEnum>){
                self.hotkeys_enable.insert(keys, function.to_string().to_string());
            }
            pub fn set_key(&mut self, key: KeysEnum){self.keys.push(key)}

            pub fn set_pressed_key(&mut self, key: KeysEnum){self.press_keys.push(key)}
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
                self.request_state= RequestState::INCOMPLETE;
                //adjust frame to take screenshot easily
                if self.screen_type.equal("FULL")  && !self.is_multi_display{
                    frame.set_minimized(true);
                }
                if self.screen_type==ScreenshotType::RECT && !self.is_multi_display{
                    frame.set_window_size(Vec2::new(0.0,0.0));
                    frame.set_window_pos(Pos2::new(0.0,-50.0));
                }
            }

            ///show image on ui based on use case
            /// @ frame and ui -> taken by egui implementation in main.rs
            pub fn show_image(&mut self, ui: &mut Ui){
                //full -> scalata al 0.6
                if self.screen_type == ScreenshotType::FULL{
                        self.image.full_ret_image.as_mut().unwrap().show_scaled(ui, 0.6);
                    }
                //se rect distingui i casi in cui ho già scelto l'area oppure no
                else{
                    if self.is_rect_choosen(){
                        self.image.custom_ret_image.as_mut().unwrap().show_scaled(ui, 0.9);
                    }else{
                        self.image.full_ret_image.as_mut().unwrap().show_scaled(ui,0.92);
                    }
                }
            }
            ///reformat frame and ui with image
            /// screen_type==FULL-> give attention to user to reopen frame
            /// screen_type==RECT -> maximize frame with the full image shown
            /// screen_type==RECT && rect_choosen -> almost 100%, resize frame
            /// @ frame and ui taken from main.rs
            pub fn ui_with_image(&mut self, frame: &mut eframe::Frame, ui: &mut Ui){
                    self.show_image(ui);
                //se full informo l'utente e ridimensiono al 70%
                if self.screen_type.equal("FULL"){
                    self.image_show=true;
                    let original_size = self.image.full_ret_image.as_mut().unwrap().size_vec2();
                    frame.set_window_size(Vec2::new(original_size.x*0.65, original_size.y*0.65));
                    frame.request_user_attention(UserAttentionType::Informational);
                }
                //se tipo screenshot rect e ancora non ho scelto l'area massimizzo il frame
                else if self.screen_type==ScreenshotType::RECT && !self.is_rect_choosen(){
                    self.image_show=true;
                   frame.set_maximized(true);
                }
                //se rect e ho deciso l'area rendo il frame un po' più grande dell'immagine
                else if self.screen_type==ScreenshotType::RECT && self.is_rect_choosen(){
                    self.image_show=true;
                    let (width,height) = (self.image_raw.as_mut().unwrap().width(),self.image_raw.as_mut().unwrap().height());
                    if width<600 || height<600{
                        if width<600 && height<600{
                            frame.set_window_size(Vec2::new(600.0,600.0));
                        }else if width<600 && height>600{
                            frame.set_window_size(Vec2::new(600.0, height as f32))
                        }else if width>600 && height<600{
                            frame.set_window_size(Vec2::new(width as f32, 600.0))
                        }
                    }else{
                        frame.set_window_size(Vec2::new(width as f32*1.1, height as f32*1.3));
                    }
                    frame.set_centered();
                }
            }
            ///state-change from incomplete based on use case
            ///take the screenshot in case of full or rect mode of single display
            /// going in choice monitor if multi display usage
            pub fn process_incomplete_request(&mut self){
                self.erased =false;
                if !self.is_multi_display{
                    //caso full
                    if self.screen_type.equal("FULL"){
                        self.full_screenshot(&screen_utils::get_screen(0));
                        self.set_request_state(RequestState::PROCESSED);
                    }
                    //caso rect
                    else{
                        self.full_screenshot(&screen_utils::get_screen(0));
                        self.set_request_state(RequestState::CHOICE_RECT);
                    }
                }else{
                    self.request_state = RequestState::CHOICE_MONITOR;
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
                    self.reinit_app();
                    self.erased=true;
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
                match function {
                    HotkeysFunctions::NewFull => {
                        if self.request_state == RequestState::INITIALIZED{
                            self.screen_request_init(frame);
                        }
                    }
                    HotkeysFunctions::NewCustom => {}
                    HotkeysFunctions::QuarterTopRight => {}
                    HotkeysFunctions::QuarterTopLeft => {}
                    HotkeysFunctions::QuarterDownRight => {}
                    HotkeysFunctions::QuarterDownLeft => {}
                }
            }

            pub fn clear_keys(&mut self){
                self.keys.clear();
            }
            pub fn clear_press_keys(&mut self){
                self.press_keys.clear();
            }

            //--------------------------------------------------------------------------------------
            //REINIT APP FOR HANDLING PREVIOUS STATES
            pub(crate) fn reinit_app(&mut self){
                self.request_state=RequestState::INITIALIZED;
                self.image=ImageToShow::default();
                self.image_raw= None;
                self.erased=false;
                self.screen_type= ScreenshotType::FULL;
                self.is_multi_display =false;
                self.screen_number=0;
                self.rect_positions=[Pos2::new(0.0,0.0), Pos2::new(0.0,0.0)];
                self.outside_rect=false;
                self.rect_choosen=false;
            }
        }
        //------------------------------------------------------------------------------------------
        //STATIC METHODS
        //get ra

        ///static method to initialized icons for ui
        fn init_icons() -> Vec<RetainedImage> {
            vec![
                RetainedImage::from_image_bytes(
                    "cut icon",
                    include_bytes!("../icons/scissors.png"),
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "display icon",
                    include_bytes!("../icons/display.png"),
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "erase icon",
                    include_bytes!("../icons/eraser.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "ok icon",
                    include_bytes!("../icons/ok.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "back_icon",
                    include_bytes!("../icons/back.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "folder",
                    include_bytes!("../icons/folder.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "copy",
                    include_bytes!("../icons/copy.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "more",
                    include_bytes!("../icons/more.png")
                ).unwrap(),
                RetainedImage::from_image_bytes(
                    "shortcut",
                    include_bytes!("../icons/shortcut.png")
                ).unwrap()


            ]
        }
    }

