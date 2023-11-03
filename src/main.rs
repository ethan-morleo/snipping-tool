
use eframe::{egui, HardwareAcceleration, Storage};
use egui::{Pos2, Rect};
use egui_extras::RetainedImage;
use image::{DynamicImage, RgbaImage};
use crate::app::app_utils::MyApp;
use crate::enums::app_enums::{RequestState, SavedData};
use crate::graphic::graphics::{choice_monitor_page, choice_screen_page, editing_page, home_page, hotkeys_pages, processed_page, saving_option_page};
use crate::input::input::{control_keyboard};
use crate::utils::utils::custom_fonts;


mod app;
mod components;
mod enums;
mod input;
mod utils;
mod graphic;
mod command;

fn  main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 300.0)),
        ..Default::default()
    };
     options.decorated = true;
     options.drag_and_drop_support = true;
     options.follow_system_theme = true;
     options.resizable = true;
     options.centered = true;
     options.hardware_acceleration = HardwareAcceleration::Preferred;
    eframe::run_native(
        "RUST POLITO SNIPPING TOOL",
        options,
        Box::new(|_cc| Box::<app::app_utils::MyApp>::default()),
    )
}


impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let clipboard = &mut arboard::Clipboard::new().unwrap();

        //setup the app in the first update in case of restore of user preferences
        if !self.is_setup(){
            self.setup();
            frame.set_fullscreen(false);
            self.set_setup(true);
            ctx.set_pixels_per_point(1.0);
            custom_fonts(self,ctx);
        }
        //UI
        egui::CentralPanel::default().show(ctx, |ui| {
            control_keyboard(self,ctx, frame, clipboard);
            //--------------------------------------------------------------------------------------
            //UI FOR THE APP IN INITIALIZED
            if self.get_request_state().equal("INITIALIZED"){
                home_page(ui, self, frame,ctx);
            }
            //--------------------------------------------------------------------------------------
            //UI FOR CHOOSING THE RIGHT MONITOR
            if self.get_request_state().equal("ChoiceMonitor"){
                choice_monitor_page(ui,self,frame,ctx);
            }
            //--------------------------------------------------------------------------------------
            //UI FOR CHOOSING SCREENSHOT CUSTOM AREA
            if self.get_request_state().equal("ChoiceScreen"){
                choice_screen_page(ui,self,frame,ctx);
            }
            //--------------------------------------------------------------------------------------
            //UI IN TERMINAL STATE
            if self.get_request_state().equal("PROCESSED"){
               processed_page(ui,self,frame,ctx,clipboard);
            }
            //--------------------------------------------------------------------------------------
            //EDIT IMAGE UI
            if self.get_request_state().equal("EditImage"){
               editing_page(ui,self,frame,ctx);
            }
            //--------------------------------------------------------------------------------------
            //HOTKEYS UI
            //HOTKEY VIEW WINDOW
            if self.get_request_state().equal("HotkeyWindow") ||self.get_request_state().equal("HotkeysSelection") || self.get_request_state().equal("HotkeysAdd"){
                    //UI FOR HOTKEYS EDIT WINDOW
                hotkeys_pages(ui,self,ctx);
            }
            //--------------------------------------------------------------------------------------
            //UI FOR SAVING PREFERENCES
            if self.get_request_state().equal("SavePreferences"){
                saving_option_page(ui,self,ctx);
            }
        });
        //------------------------------------------------------------------------------------------
        //state change after the invisible frame setting, no ui needed
        if self.get_request_state().equal("Reframe"){
            self.process_screen_request();
        }
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        let mut hotkeys_function = vec![];
        let mut shortcuts = vec![];
        if !self.get_hotkey_enable().is_empty(){
            for (k,v) in self.get_hotkey_enable(){
                shortcuts.push(k);
                hotkeys_function.push(v);
            }
        }
        let default = self.get_default();
        let data_to_save = SavedData::new(hotkeys_function,shortcuts,default);
        confy::store("rust-snipping-tool", None, data_to_save).unwrap()
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {
        if let Some(screenshot) = _frame.screenshot(){
            let y_to_decrease = self.get_size_button() + 24.0;
            self.erase_drawing();
            let mut window_size = _frame.info().window_info.size;
            window_size.y = window_size.y.clone() - y_to_decrease;
            let region = Rect::from_min_size(Pos2::new(0.0,y_to_decrease), window_size);
            let real_screenshot = screenshot.region(&region, Some(1.0));
            let image = RetainedImage::from_color_image(
                "edit_image",
                real_screenshot.clone()
            );
            let rgba_image = RgbaImage::from_raw(
                real_screenshot.clone().width() as u32,
                real_screenshot.clone().height() as u32,
                Vec::from(real_screenshot.clone().as_raw())
            ).unwrap();
            self.image = Some(image);
            self.image_raw = Some(DynamicImage::from(rgba_image));
            self.set_edit_image(true);
            self.set_request_state(RequestState::Processed);
        }
    }

}