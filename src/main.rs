
use eframe::egui;
use egui::{Align, CursorIcon, Layout};
use crate::draw::draw_utils::{draw_add_hotkey_combobox, draw_back_button, draw_back_menu_button, draw_combobox, draw_copy_button, draw_delete_button, draw_enable_hotkeys_shortcuts, draw_erase_button, draw_file_picker, draw_image, draw_more_menu, draw_new_button, draw_ok_button, draw_ok_shortcut_button, draw_red_rect, draw_select_hotkey, draw_shortcut_selection};
use crate::enums::app_enums::ScreenshotType;
use crate::input::input::{control_keyboard, control_mouse_input};

mod app;
mod draw;
mod enums;
mod input;
mod utils;

fn main() -> Result<(), eframe::Error> {
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


impl eframe::App for app::app_utils::MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
            //UI FOR CHOOSING SCREENSHOT CUSTOM AREA
            if self.get_request_state().equal("CHOICE_RECT"){
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
                        if !self.is_outside_rect(){
                            ctx.set_cursor_icon(CursorIcon::Crosshair);
                        }
                        draw_red_rect(self, ui);
                    }
                );
            }
            //--------------------------------------------------------------------------------------
            //UI IN TERMINAL STATE
            if self.get_request_state().equal("PROCESSED"){
                ui.horizontal(
                    |ui|{
                        draw_new_button(self,frame,ui,ctx);
                        draw_erase_button(self,ui,ctx);
                        ui.with_layout(
                            Layout::right_to_left(Align::Center),
                            |ui|{
                                draw_more_menu(self,ui,ctx);
                                draw_file_picker(self,ui,ctx);
                                draw_copy_button(self,ui,ctx,clipboard);
                            }
                        );
                        ui.separator();
                    });

                if self.get_screen_type()==ScreenshotType::RECT {ui.add_space(20.0);}

                draw_image(self, frame,ui);
            }
            //--------------------------------------------------------------------------------------
            //HOTKEYS UI

            //HOTKEY VIEW WINDOW
            if self.get_request_state().equal("HOTKEY_WINDOW") ||self.get_request_state().equal("HOTKEYS_SELECTION") {
                    //UI FOR HOTKEY SELECTION TYPE WINDOW
                    if self.get_request_state().equal("HOTKEY_WINDOW"){
                        ui.vertical(
                            |ui|{
                                draw_enable_hotkeys_shortcuts(self,ui);
                            }
                        );
                        ui.with_layout(
                            Layout::bottom_up(Align::Center),
                            |ui|{
                                ui.horizontal(
                                    |ui|{
                                        draw_add_hotkey_combobox(self,ui);
                                        draw_select_hotkey(self,ui);
                                        draw_back_menu_button(self,ui);
                                    }
                                );
                                ui.add_space(20.0);
                                ui.label("SCEGLI TRA LE FUNZIONI E SETTA LE SHORTCUTS");
                            }
                        );
                    }else{
                        //UI FOR HOTKEY SELECTION SHORTCUT WINDOW
                        draw_shortcut_selection(self,ui);
                        ui.add_space(30.0);
                        ui.horizontal(
                            |ui|{
                                draw_ok_shortcut_button(self, ui);
                                draw_delete_button(self, ui);
                            }
                        );
                    }
            }
        });
    }
}