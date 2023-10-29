
use eframe::{egui, HardwareAcceleration, Storage};
use egui::{Align,CursorIcon, Layout, Pos2, Rect, UserAttentionType, Vec2};
use egui_extras::RetainedImage;
use image::{DynamicImage, RgbaImage};
use itertools::Itertools;
use crate::app::app_utils::MyApp;
use crate::draw::draw_utils::{draw_add_hotkey_combobox, draw_all_paintings, draw_back_button, draw_back_menu_button, draw_color_picker_button, draw_combobox, draw_copy_button, draw_delay_combobox, draw_delete_button, draw_delete_function_button, draw_edit_button, draw_enable_hotkeys_shortcuts, draw_erase_button, draw_file_picker, draw_image, draw_monitor_button, draw_more_menu, draw_new_button, draw_ok_button, draw_ok_shortcut_button, draw_painting_combobox, draw_red_rect, draw_save_folder, draw_select_hotkey, draw_shortcut_selection, draw_text_edit, ok_default_button};
use crate::enums::app_enums::{RequestState, SavedData, ScreenshotType};
use crate::input::input::{control_keyboard, control_mouse_input};
use crate::utils::utils::{get_possible_hotkeys_functions, set_cursor};



mod app;
mod draw;
mod enums;
mod input;
mod utils;
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
        //setup the app in the first update in case of restore of user preferences
        if !self.is_setup(){
            self.setup();
            self.set_setup(true);
        }

        let clipboard = &mut arboard::Clipboard::new().unwrap();

        //------------------------------------------------------------------------------------------
        //state change from incomplete, no ui needed
        if self.get_request_state().equal("INCOMPLETE"){
           self.process_incomplete_request();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            control_keyboard(self,ctx, frame, clipboard);
            //--------------------------------------------------------------------------------------
            //UI FOR THE APP IN INITIALIZED
            if self.get_request_state().equal("INITIALIZED"){
                ui.with_layout(Layout::top_down(Align::LEFT), |ui|{
                    ui.horizontal(|ui|{
                        draw_new_button(self, frame, ui, ctx);
                        draw_combobox(self,ui);
                        ui.separator();
                        ui.label("DELAY: ");
                        draw_delay_combobox(self, ui);
                        ui.with_layout(
                            Layout::right_to_left(Align::Center),
                            |ui|{
                                draw_more_menu(self,ui,ctx);
                            }
                        )
                    }
                    );
                });
                ui.separator();
            }
            //--------------------------------------------------------------------------------------
            //UI FOR CHOOSING THE RIGHT MONITOR
            if self.get_request_state().equal("ChoiceMonitor"){
                for i in 1..self.get_display_number()+1{
                    ui.add_space(30.0);
                    ui.horizontal_centered(|ui|{
                        draw_monitor_button(self,ui,ctx,i, frame);
                    });
                }
            }
            //--------------------------------------------------------------------------------------
            //UI FOR CHOOSING SCREENSHOT CUSTOM AREA
            if self.get_request_state().equal("ChoiceRect"){
                ui.horizontal(
                    |ui|{
                        draw_image(self, frame,ui);
                        control_mouse_input(self, ctx);
                        ui.add_space(15.0);
                        ui.vertical(
                          |ui|{
                              draw_ok_button(self,ui,ctx, frame);
                              draw_back_button(self,ui,ctx);
                          }
                        );
                        if !self.is_outside_rect() && !self.is_rect_shown(){
                            ctx.set_cursor_icon(CursorIcon::Crosshair);
                        } else if self.is_rect_shown(){
                            set_cursor(self, ctx);
                        }

                        draw_red_rect(self, ui);
                    }
                );
            }
            //--------------------------------------------------------------------------------------
            //UI IN TERMINAL STATE
            if self.get_request_state().equal("PROCESSED"){
                set_cursor(self, ctx);

                ui.horizontal(
                    |ui|{
                        draw_new_button(self,frame,ui,ctx);
                        ui.separator();
                        ui.label("DELAY");
                        draw_delay_combobox(self,ui);
                        ui.separator();
                        draw_erase_button(self,ui,ctx);
                        draw_edit_button(self,ui,ctx);
                        draw_file_picker(self,ui,ctx);
                        draw_copy_button(self,ui,ctx,clipboard);
                        ui.with_layout(
                            Layout::right_to_left(Align::Center),
                            |ui|{
                                draw_more_menu(self,ui,ctx);
                                ui.separator();
                            }
                        );
                    });
                ui.separator();
                if self.get_screen_type()==ScreenshotType::CUSTOM {ui.add_space(10.0);}
                draw_image(self, frame,ui);
                if frame.info().window_info.size.x == 0.0 && frame.info().window_info.size.y == 0.0{
                    frame.request_user_attention(UserAttentionType::Informational);
                }
                ui.add_space(5.0);
            }
            //--------------------------------------------------------------------------------------
            //EDIT IMAGE UI
            if self.get_request_state().equal("EditImage"){
                control_mouse_input(self, ctx);
                ui.horizontal(
                    |ui|{
                        draw_back_button(self, ui, ctx);
                        draw_painting_combobox(self, ui);
                        draw_color_picker_button(self, ui);
                        ui.separator();
                        draw_erase_button(self, ui, ctx);
                        if ui.add(egui::ImageButton::new(
                            self.get_icon(12).texture_id(ctx), Vec2::new(30.0,30.0)
                        )).clicked(){
                            self.editing = None;
                            frame.request_screenshot();
                        }
                    }
                );
                ui.separator();
                draw_image(self, frame, ui);
                draw_all_paintings(self, ui);
            }
            //--------------------------------------------------------------------------------------
            //HOTKEYS UI
            //HOTKEY VIEW WINDOW
            if self.get_request_state().equal("HotkeyWindow") ||self.get_request_state().equal("HotkeysSelection") || self.get_request_state().equal("HotkeysAdd"){
                    //UI FOR HOTKEYS EDIT WINDOW
                    if self.get_request_state().equal("HotkeyWindow") || self.get_request_state().equal("HotkeysAdd"){
                        ui.vertical(
                            |ui|{
                                    draw_enable_hotkeys_shortcuts(self,ui);
                            }
                        );
                        ui.with_layout(
                            Layout::bottom_up(Align::LEFT),
                            |ui|{
                                    ui.add_space(30.0);
                                    ui.horizontal(
                                        |ui|{
                                            if self.get_request_state().equal("HotkeysAdd"){
                                                draw_add_hotkey_combobox(self,ui);
                                                draw_select_hotkey(self, ui);
                                            }else{
                                                if !get_possible_hotkeys_functions(self.get_hotkey_enable()).is_empty(){
                                                    if ui.button("ADD").clicked(){
                                                        self.set_request_state(RequestState::HotkeysAdd);
                                                    }
                                                }

                                            }
                                            draw_back_menu_button(self,ui);
                                        });
                                ui.separator();
                                ui.label("SCEGLI TRA LE FUNZIONI E SETTA LE SHORTCUTS");
                            }
                        );
                    }else{
                        //UI FOR HOTKEY SELECTION SHORTCUT WINDOW
                        draw_shortcut_selection(self,ui);
                        ui.add_space(30.0);
                        if !self.get_press_keys().is_empty() {
                            if self.is_repeated_keys(){
                                ui.label("ALREADY EXISTING HOTKEYS, IF PRESS OK WOULD BE OVERWRITING");
                            }
                            ui.horizontal(
                                |ui| {
                                    draw_ok_shortcut_button(self, ui);
                                    draw_delete_button(self, ui);
                                }
                            );
                            ui.add_space(20.0);
                        };
                        ui.separator();
                        ui.horizontal(
                            |ui|{
                                draw_back_menu_button(self, ui);
                                if self.get_hotkey_enable().values().contains(&self.get_hotkey_selected().to_string().to_string()){
                                    ui.separator();
                                    draw_delete_function_button(self,ui);
                                }
                            }
                        );
                    }
            }
            //--------------------------------------------------------------------------------------
            //UI FOR SAVING PREFERENCES
            if self.get_request_state().equal("SavePreferences"){
                draw_save_folder(self, ui);
                ui.add_space(30.0);
                draw_text_edit(self, ui);
                ui.add_space(50.0);
                ok_default_button(self, ui);
            }
        });
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
            self.erase_drawing();
            let mut window_size = _frame.info().window_info.size;
            window_size.y = window_size.y.clone() -54.0;
            let region = Rect::from_min_size(Pos2::new(0.0,54.0), window_size);
            let real_screenshot = screenshot.region(&region,_frame.info().native_pixels_per_point);
            let image = RetainedImage::from_color_image(
                "edit_image",
                real_screenshot.clone()
            );
            let rgba_image = RgbaImage::from_raw(
                real_screenshot.clone().width() as u32,
                real_screenshot.clone().height() as u32,
                Vec::from(real_screenshot.clone().as_raw())
            ).unwrap();
            self.set_image_to_show(image);
            self.image_raw = Some( DynamicImage::from(rgba_image));
            self.set_edit_image(true);
            self.set_request_state(RequestState::Processed);
        }
    }

}