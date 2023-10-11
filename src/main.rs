
use eframe::{egui, Storage};
use egui::{Align, CursorIcon, Layout, Pos2};
use itertools::Itertools;
use serde::Serialize;
use crate::app::app_utils::MyApp;
use crate::draw::draw_utils::{draw_add_hotkey_combobox, draw_back_button, draw_back_menu_button, draw_combobox, draw_copy_button, draw_delete_button, draw_delete_function_button, draw_edit_button, draw_enable_hotkeys_shortcuts, draw_erase_button, draw_file_picker, draw_image, draw_monitor_button, draw_more_menu, draw_new_button, draw_ok_button, draw_ok_shortcut_button, draw_paint_button, draw_red_rect, draw_save_folder, draw_select_hotkey, draw_shortcut_selection, draw_text_button, draw_text_edit, ok_default_button};
use crate::enums::app_enums::{RequestState, ScreenshotType};
use crate::input::input::{control_keyboard, control_mouse_input};
use crate::utils::utils::{set_cursor};



mod app;
mod draw;
mod enums;
mod input;
mod utils;


 fn  main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 300.0)),
        ..Default::default()
    };
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
        frame.set_decorations(true);
        ctx.set_pixels_per_point(1.0);
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
                ui.horizontal(|ui|{
                    draw_new_button(self, frame, ui, ctx);
                    draw_combobox(self,ui);
                    ui.with_layout(
                        Layout::right_to_left(Align::Center),
                        |ui|{
                            draw_more_menu(self,ui,ctx);
                        }
                    )
                });
                ui.separator();
            }
            //--------------------------------------------------------------------------------------
            //UI FOR CHOOSING THE RIGHT MONITOR
            if self.get_request_state().equal("ChoiceMonitor"){
                for i in 1..self.get_display_number()+1{
                    ui.add_space(30.0);
                    draw_monitor_button(self,ui,ctx,i, frame);
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
                              draw_ok_button(self,ui,ctx);
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
                        draw_erase_button(self,ui,ctx);
                        draw_edit_button(self,ui,ctx);

                        ui.with_layout(
                            Layout::right_to_left(Align::Center),
                            |ui|{
                                draw_more_menu(self,ui,ctx);
                                draw_file_picker(self,ui,ctx);
                                draw_copy_button(self,ui,ctx,clipboard);
                            }
                        );
                    });
                ui.separator();
                if self.get_screen_type()==ScreenshotType::CUSTOM {ui.add_space(20.0);}
                draw_image(self, frame,ui);
            }
            //--------------------------------------------------------------------------------------
            //EDIT IMAGE UI
            if self.get_request_state().equal("EditImage"){
                if self.get_edit_type().is_some() {
                    if (self.get_edit_type().unwrap().equal("Text") && self.get_edit_position() == [Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)]) || self.get_edit_type().unwrap().equal("Painting") {
                        control_mouse_input(self,ctx);
                    }
                }
                ui.horizontal(
                    |ui|{
                        draw_text_button(self,ui,ctx);
                        draw_paint_button(self,ui,ctx);
                        draw_back_button(self,ui,ctx);
                    }
                );
                ui.separator();
                ui.add_space(10.0);
                draw_image(self,frame,ui);
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
                                    ui.horizontal(
                                        |ui|{
                                            if self.get_request_state().equal("HotkeysAdd"){
                                                draw_add_hotkey_combobox(self,ui);
                                                draw_select_hotkey(self, ui);
                                            }else{
                                                if ui.button("ADD").clicked(){
                                                    self.set_request_state(RequestState::HotkeysAdd);
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
                        if !self.get_keys().is_empty() {
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
        let data_to_save = self.get_saved_data();
        confy::store("rust-snipping-tool", None, data_to_save).unwrap()
    }

}