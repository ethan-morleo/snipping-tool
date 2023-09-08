
use eframe::egui;
use egui::{Align, Color32, ColorImage, CursorIcon, Key, Layout, PointerButton, Pos2, Rect, Rounding, Stroke, TextureId, Ui, UserAttentionType, Vec2, Visuals};
use egui::accesskit::Role::Image;
use egui::WidgetType::Button;
use egui_extras::RetainedImage;
use image::{ImageBuffer, RgbaImage};
use screenshots::{Compression, Screen};
mod mods;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "RUST POLITO SNIPPING TOOL",
        options,
        Box::new(|_cc| Box::<mods::app_utils::MyApp>::default()),
    )
}


impl eframe::App for mods::app_utils::MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.set_decorations(true);
        ctx.set_pixels_per_point(1.0);
        //------------------------------------------------------------------------------------------
        //state change from incomplete, no ui needed
        if self.get_request_state().equal("INCOMPLETE"){
           self.process_incomplete_request();
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            //--------------------------------------------------------------------------------------
            //UI FOR THE APP IN INITIALIZED
            if self.get_request_state().equal("INITIALIZED"){
                ui.horizontal(|ui|{
                    self.draw_new_button(frame,ui, ctx); //new button: state change in incomplete and started screenshot flow
                    self.draw_combobox(ui); //choose mode (FULL OR RECT)
                });
            }
            //--------------------------------------------------------------------------------------
            //UI FOR CHOOSING SCREENSHOT CUSTOM AREA
            if self.get_request_state().equal("CHOICE_RECT"){
                ui.horizontal(
                    |ui|{
                        self.draw_image(frame,ui);
                        self.control_mouse_input(ctx);
                        ui.add_space(15.0);
                        ui.vertical(
                          |ui|{
                              self.draw_ok_button(frame,ui,ctx);
                              self.draw_back_button(frame,ui,ctx);
                          }
                        );
                        if !self.is_outside_rect(){
                            ctx.set_cursor_icon(CursorIcon::Crosshair);
                        }
                        self.draw_red_rect(ui);
                    }
                );
            }
            //--------------------------------------------------------------------------------------
            //UI IN TERMINAL STATE
            if self.get_request_state().equal("PROCESSED"){
                ui.horizontal(
                    |ui|{
                        self.draw_new_button(frame,ui,ctx);
                        self.draw_erase_button(frame,ui,ctx);
                        self.draw_file_picker(frame,ui,ctx);
                    });
                self.draw_image(frame,ui);
            }

        });
    }
}