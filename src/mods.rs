    pub(crate) mod screen_utils{
        use std::path::Display;
        use egui::{ColorImage, Vec2};
        use egui_extras::RetainedImage;
        use image::{DynamicImage, ImageBuffer, RgbaImage};
        use screenshots::{Compression, Image, Screen};

        pub fn get_screen_number() -> usize{ Screen::all().unwrap().len() }

        pub fn get_screen(number:usize) -> Screen{ Screen::all().unwrap().get(number).expect("No screen retrieved").clone() }

        pub fn get_screen_from_pos(x:i32, y:i32) -> Screen{
            Screen::from_point(x,y).unwrap()
        }

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
        use std::cmp::max;
        use std::ops::Deref;
        use eframe;
        use egui;
        use egui::{Color32, ColorImage, Pos2, Rect, Rounding, Stroke, Ui, UserAttentionType, Vec2};
        use egui_extras::RetainedImage;
        use screenshots::{DisplayInfo, Image, Screen};
        use crate::mods::screen_utils;
        use image;
        use image::{DynamicImage, GenericImageView, ImageBuffer, RgbaImage};
        use image::imageops::FilterType;

        ///enum for all screenshot type
        #[derive(Debug, PartialEq, Copy, Clone)]
        pub(crate) enum ScreenshotType{
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
        pub(crate) enum RequestState{
            INITIALIZED, //non ho premuto il tasto +
            INCOMPLETE, //caso in cui ho premuto il tasto + e devo fare lo screen
            CHOICE_RECT, //ho fatto lo screen per la fake trasparenza devo scegliere il rettangolo
            CHOICE_MONITOR, //caso di multi display sto alla schermata di scelta del monitor
            PROCESSED, //ho terminato la richiesta
        }
        impl RequestState{
            pub fn equal(self, state:&str) -> bool {
                match state{
                    "INITIALIZED" =>{self ==RequestState::INITIALIZED},
                    "INCOMPLETE" =>{self ==RequestState::INCOMPLETE},
                    "CHOICE_RECT" =>{self == RequestState::CHOICE_RECT},
                    "CHOICE_MONITOR" =>{self == RequestState::CHOICE_MONITOR},
                    "PROCESSED" =>{self == RequestState::PROCESSED},
                    _ => {panic!("INVALID STATE IN INPUT")}
                }
            }
        }
        struct ImageToShow{
            full_ret_image: Option<RetainedImage>,
            custom_ret_image: Option<RetainedImage>
        }
        impl Default for ImageToShow{
            fn default() -> Self {
                Self{
                    full_ret_image: None,
                    custom_ret_image: None
                }
            }
        }
        pub(crate) struct MyApp {
            request_state: RequestState,
            image: ImageToShow,
            image_raw: Option<DynamicImage>,
            image_show:bool,
            erased:bool,
            screen_type: ScreenshotType,
            is_multi_display: bool,
            screen_number: usize,
            rect_positions: [Pos2;2],
            outside_rect: bool,
            rect_choosen: bool,
            icons: Vec<RetainedImage>

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
        ///methods declaration for struct myApp
        impl MyApp{
            //--------------------------------------------------------------------------------------
            //GETTER
            pub fn get_request_state(&self) ->RequestState{
                self.request_state
            }
            pub fn get_screen_type(&self) -> ScreenshotType{
                self.screen_type
            }
            pub fn get_screen_number(&self) -> usize{
                self.screen_number
            }
            pub fn get_icon(&self, index:usize) -> &RetainedImage {
                self.icons.get(index).unwrap()
            }
            pub fn get_rect_position(&self) -> [Pos2;2]{ self.rect_positions }

            pub fn is_multi_display(&self) -> bool{
                self.is_multi_display
            }
            pub fn is_erased(&self) -> bool{
                self.erased
            }
            pub fn is_outside_rect(&self) -> bool{self.outside_rect}
            pub fn is_rect_choosen(&self) ->bool{self.rect_choosen}
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
            pub fn set_outside_rect(&mut self, is_outside_rect: bool){ self.outside_rect = is_outside_rect;}
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
                if self.screen_type == ScreenshotType::FULL && !self.is_multi_display{
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
                if self.screen_type == ScreenshotType::FULL{
                    self.image_show=true;
                    let original_size = self.image.full_ret_image.as_mut().unwrap().size_vec2();
                    frame.set_window_size(Vec2::new(original_size.x*0.65, original_size.y*0.65));
                    frame.request_user_attention(UserAttentionType::Informational);
                }
                //se rect e ancora non ho scelto l'area massimizzo il frame
                else if self.screen_type==ScreenshotType::RECT && !self.is_rect_choosen(){
                    self.image_show=true;
                   frame.set_maximized(true);
                }
                //se rect e ho deciso l'area rendo il frame un po' più grande dell'immagine
                else if self.screen_type==ScreenshotType::RECT && self.is_rect_choosen(){
                    self.image_show=true;
                    let (width,height) = (self.image_raw.as_mut().unwrap().width(),self.image_raw.as_mut().unwrap().height());
                    if width<300 || height<300{
                        if width<300 && height<300{
                            frame.set_window_size(Vec2::new(300.0,300.0));
                        }else if width<300 && height>300{
                            frame.set_window_size(Vec2::new(300.0, height as f32))
                        }else{
                            frame.set_window_size(Vec2::new(width as f32, 300.0))
                        }
                    }else{
                        frame.set_window_size(Vec2::new(width as f32*1.1, height as f32*1.1));
                    }
                    frame.set_centered();
                }
            }
            ///right change state from incomplete based on use case
            ///take the screenshot in case of full or rect mode of single display
            /// going in choice monitor if multi display usage
            pub fn process_incomplete_request(&mut self){
                self.erased =false;
                if self.screen_type == ScreenshotType::FULL{
                    if !self.is_multi_display{
                        self.full_screenshot(&screen_utils::get_screen(0));
                        self.set_request_state(RequestState::PROCESSED);
                    }else{
                        self.request_state = RequestState::CHOICE_MONITOR;
                    }
                }
                //caso rect
                else {
                    //caso singolo monitor vado in choice rect
                    if !self.is_multi_display{
                        self.full_screenshot(&screen_utils::get_screen(0));
                        self.set_request_state(RequestState::CHOICE_RECT);
                    }else{
                        self.request_state = RequestState::CHOICE_MONITOR;
                    }
                }
            }
            pub fn is_rect_shown(&self)->bool{
                //se vedo il rettangolo rosso i due elementi sono diversi tra loro e non sono entrambi uguali a 0
                self.rect_positions[0]!=Pos2::new(0.0,0.0) && self.rect_positions[1]!= Pos2::new(0.0,0.0)
            }

            pub fn replace_image_with_rect(&mut self){
                self.normalize_coords(screen_utils::get_screen(0).display_info);
                self.image_raw =Some(self.image_raw.as_mut().unwrap().crop_imm(
                    (self.rect_positions[0].x) as u32,
                    (self.rect_positions[0].y) as u32,
                    //TODO: modificare in base alla direzione
                    (self.rect_positions[1].x - self.rect_positions[0].x) as u32,
                    (self.rect_positions[1].y - self.rect_positions[0].y) as u32
                ));
                self.image.custom_ret_image=retained_image_from_dynamic(self.image_raw.as_ref().unwrap());
            }

            ///take control of the go back flow for the frontend
            pub fn go_back(&mut self){
                    self.reinit_app();
                    self.erased=true;
            }

            pub fn normalize_coords(&mut self, disp_info:DisplayInfo){
                let x_aspect_ratio: f32= (disp_info.width as f32/((disp_info.width as f32*0.92)+12.0));
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

            //--------------------------------------------------------------------------------------
            // DRAW UI METHODS
            ///draw add button
            pub fn draw_new_button(&mut self, frame:&mut eframe::Frame, ui: &mut Ui, ctx: &egui::Context){
                ui.add_space(30.0);
                //new button
                if ui.add(egui::Button::image_and_text(
                    self.get_icon(0).texture_id(ctx),
                    Vec2::new(30.0, 30.0),
                    "NEW"
                )).clicked(){
                    if self.request_state==RequestState::INITIALIZED{
                        self.screen_request_init(frame);
                    }else{
                        let screen_type = self.screen_type;
                        self.reinit_app();
                        self.erased=true;
                        self.screen_type=screen_type;
                        self.screen_request_init(frame);

                    }

                }
            }
            ///draw combobox
            pub fn draw_combobox(&mut self, ui: &mut Ui){
                ui.add_space(20.0);
                egui::ComboBox::new("mode", "")
                    .selected_text(format!("{:?}", self.get_screen_type()))
                    .show_ui(ui, |ui|{
                        for screen_type in [
                            ScreenshotType::FULL,
                            ScreenshotType::RECT,
                        ] {
                            ui.selectable_value(&mut self.screen_type, screen_type, format!("{:?}", screen_type));
                        }
                    });
            }
            ///draw erase button
            pub fn draw_erase_button(&mut self, frame:&mut eframe::Frame, ui: &mut Ui, ctx: &egui::Context){
                ui.add_space(20.0);
                //new button
                if ui.add(egui::Button::image_and_text(
                    self.get_icon(2).texture_id(ctx),
                    Vec2::new(30.0, 30.0),
                    "DISCARD"
                )).clicked(){
                    self.go_back();
                }
            }
            ///draw image on ui differently based on use case
            pub fn draw_image(&mut self, frame: &mut eframe::Frame, ui:&mut Ui){
                if !self.erased{
                    if self.screen_type==ScreenshotType::FULL || self.is_rect_choosen(){
                        ui.vertical_centered(
                            |ui|{
                                self.ui_with_image(frame,ui);
                            });
                    }else{
                        self.ui_with_image(frame, ui);
                    }
                }
            }
            ///draw ok button to confirm custom rect screen
            pub fn draw_ok_button(&mut self, frame:&mut eframe::Frame, ui: &mut Ui, ctx: &egui::Context){
                ui.add_space(30.0);
                if ui.add(egui::ImageButton::new(
                    self.get_icon(3).texture_id(ctx),
                    Vec2::new(70.0,70.0)
                )).clicked(){
                    if self.is_rect_shown(){
                        self.replace_image_with_rect();
                        self.rect_choosen = true;
                        self.set_request_state(RequestState::PROCESSED); //transition to final state
                    }
                }
            }
            ///draw back button that control the go back frontend flow
            pub fn draw_back_button(&mut self, frame:&mut eframe::Frame, ui: &mut Ui, ctx: &egui::Context){
                ui.add_space(20.0);
                if ui.add(
                    egui::ImageButton::new(
                        self.get_icon(4).texture_id(ctx),
                        Vec2::new(70.0,70.0)
                    )
                ).clicked(){
                    self.go_back();

                }
            }
            pub fn draw_centered_label(&self, ui: &mut Ui, text:&str){
                ui.vertical_centered(
                    |ui|{
                        ui.label(text);
                    }
                );
            }
            pub fn draw_red_rect(&self, ui: &mut Ui){
                ui.painter().rect(
                    Rect::from_two_pos(self.get_rect_position()[0], self.get_rect_position()[1]),
                    Rounding::none(),
                    Color32::from_rgba_unmultiplied(220, 220, 220, 9 as u8),
                    Stroke::new(1.5,Color32::RED)
                );
            }
            //--------------------------------------------------------------------------------------
            pub fn control_mouse_input(&mut self, ctx: & egui::Context){
                ctx.input(
                    |i|{
                        //ho iniziato a fare la diagonale del rettangolo
                        //controllo che il mouse sia dentro l'immagine
                        if !(i.pointer.hover_pos().is_some() && i.pointer.hover_pos().unwrap().x>((self.image.full_ret_image.as_mut().unwrap().width() as f32*0.92) + 12.0)){
                            self.set_outside_rect(false);
                            if i.pointer.primary_pressed(){
                                self.set_rect_position(1,i.pointer.press_origin().unwrap());
                                self.set_rect_position(2,i.pointer.press_origin().unwrap());
                            }
                            if i.pointer.is_decidedly_dragging(){
                                if i.pointer.primary_down(){
                                    self.set_rect_position(2, i.pointer.interact_pos().unwrap());
                                }
                            }
                        }else{
                            self.set_outside_rect(true);
                        }
                    }
                );
            }
            //--------------------------------------------------------------------------------------
            fn reinit_app(&mut self){
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
        //STATIC METHOD
        pub fn retained_image_from_dynamic(dyn_image:&DynamicImage) -> Option<RetainedImage> {
            Some(RetainedImage::from_color_image(
                "screen", ColorImage::from_rgba_unmultiplied(
                    [dyn_image.width() as _, dyn_image.height() as _],
                    dyn_image.as_bytes()
                )))
        }
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
                ).unwrap()
            ]
        }
    }

