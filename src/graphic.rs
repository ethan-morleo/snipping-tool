pub(crate) mod graphics{
    use std::thread;
    use std::time::Duration;
    use arboard::Clipboard;
use egui::{Align, CursorIcon, Layout, Ui, UserAttentionType, Vec2};
use itertools::Itertools;
use crate::app::app_utils::MyApp;
use crate::app::screen_utils::get_screen;
use crate::components::components::{select_hotkey_button, add_hotkey_combobox, delay_combobox, delete_shortcut_button, draw_all_paintings, back_button, color_picker_combobox, copy_button, edit_button, image_on_screen, painting_combobox, draw_red_rect, draw_text_edit, file_save_button, monitor_button, more_menu, new_button, ok_default_save_button, ok_shortcut_button, save_folder_button, screen_selected_button, shortcut_selection, show_enable_hotkeys_shortcuts, reset_button, back_shortcut_button, erase_button};
use crate::enums::app_enums::RequestState;
use crate::input::input::control_mouse_input;
use crate::utils::utils::{get_possible_hotkeys_functions, set_cursor};

//HOME PAGE
pub fn home_page(ui: &mut Ui, app: &mut MyApp, frame: &mut eframe::Frame, ctx: &egui::Context ){
        ui.with_layout(Layout::top_down(Align::LEFT), |ui|{
            ui.horizontal(|ui|{
                new_button(app, frame, ui, ctx);
                ui.separator();
                ui.label("DELAY: ");
                delay_combobox(app, ui);
                ui.with_layout(
                    Layout::right_to_left(Align::Center),
                    |ui|{
                        more_menu(app, ui, ctx);
                    }
                )
            }
            );
        });
        ui.separator();
    }

//CHOICE MONITOR PAGE
pub fn choice_monitor_page(ui: &mut Ui, app: &mut MyApp, frame: &mut eframe::Frame, ctx: &egui::Context ){
        ui.label("SCEGLI IL MONITOR DA ACQUISIRE");
        ui.separator();
        ui.add_space(30.0);
        for i in 1..app.get_display_number()+1{
            ui.horizontal_centered(|ui|{
                monitor_button(app, ui, ctx, i, frame);
            });
        }
    }
//CHOICE SCREEN PAGE
pub fn choice_screen_page(ui: &mut Ui, app: &mut MyApp, frame: &mut eframe::Frame, ctx: &egui::Context ){
    if !app.is_screen_made(){
        if app.get_delay() !=0{
            thread::sleep(Duration::from_secs(app.get_delay() as u64))
        }
        app.full_screenshot(&get_screen(app.get_screen_selected()));
        app.set_screen_made(true);
    }else{
        if app.is_image_show() || app.is_full_screen_request(){
            frame.set_visible(true);
        }
        if app.is_full_screen_request(){
            app.set_rect_choosen(true);
            app.set_request_state(RequestState::Processed);
            app.set_full_screen_request(false);
        }else{
            ui.horizontal(
                |ui|{
                    image_on_screen(app, frame, ui);
                    control_mouse_input(app, ctx);
                    ui.add_space(15.0);
                    ui.vertical(
                        |ui|{
                            screen_selected_button(app, ui, ctx, frame);
                            back_button(app, ui, ctx);
                        }
                );
                    if !app.is_outside_rect() && !app.is_rect_shown(){
                        ctx.set_cursor_icon(CursorIcon::Crosshair);
                    } else if app.is_rect_shown(){
                        set_cursor(app, ctx);
                    }
                    draw_red_rect(app, ui);
                }
        );
        }
    }
}
//PROCESSED PAGE
pub fn processed_page(ui: &mut Ui, app: &mut MyApp, frame: &mut eframe::Frame, ctx: &egui::Context, clipboard: &mut Clipboard){
    set_cursor(app, ctx);
    if !app.is_screen_made(){
        app.full_screenshot(&get_screen(app.get_screen_selected()));
        app.set_screen_made(true);
    }else{
        ui.horizontal(
            |ui|{
                back_button(app, ui, ctx);
                new_button(app, frame, ui, ctx);
                ui.separator();
                ui.label("DELAY: ");
                delay_combobox(app, ui);
                ui.separator();
                edit_button(app, ui, ctx);
                file_save_button(app, ui, ctx);
                copy_button(app, ui, ctx, clipboard);
                ui.with_layout(
                    Layout::right_to_left(Align::Center),
                    |ui|{
                        more_menu(app, ui, ctx);
                        ui.separator();
                    }
                );
            });
        ui.separator();
        image_on_screen(app, frame, ui);
        if frame.info().window_info.size.x == 0.0 && frame.info().window_info.size.y == 0.0{
            frame.request_user_attention(UserAttentionType::Informational);
        }
        ui.add_space(5.0);
    }
}
//HOTKEYS PAGES
pub fn hotkeys_pages(ui: &mut Ui, app: &mut MyApp, ctx: &egui::Context){
    if app.get_request_state().equal("HotkeyWindow") || app.get_request_state().equal("HotkeysAdd"){
        ui.vertical(
            |ui|{
                show_enable_hotkeys_shortcuts(app, ui);
            }
        );
        ui.with_layout(
            Layout::bottom_up(Align::LEFT),
            |ui|{
                ui.add_space(30.0);
                ui.horizontal(
                    |ui|{
                        if app.get_request_state().equal("HotkeysAdd"){
                            add_hotkey_combobox(app, ui);
                            select_hotkey_button(app, ui);
                        }else{
                            if !get_possible_hotkeys_functions(app.get_hotkey_enable()).is_empty(){
                                if ui.button("ADD").clicked(){
                                    app.set_request_state(RequestState::HotkeysAdd);
                                }
                            }

                        }
                        back_shortcut_button(app, ui, ctx);
                    });
                ui.separator();
                ui.label("SCEGLI TRA LE FUNZIONI E SETTA LE SHORTCUTS");
            }
        );
    }else{
        //UI FOR HOTKEY SELECTION SHORTCUT WINDOW
        shortcut_selection(app, ui);
        ui.add_space(30.0);
        if !app.get_press_keys().is_empty() {
            if app.is_repeated_keys(){
                ui.label("HOTKEY GIA' PRESENTE, SE PREMI OK LA SOVRASCRIVERAI");
            }
            ui.horizontal(
                |ui| {
                    ok_shortcut_button(app, ui);
                    reset_button(app, ui);
                }
            );
            ui.add_space(20.0);
        };
        ui.separator();
        ui.horizontal(
            |ui|{
                back_shortcut_button(app, ui, ctx);
                if app.get_hotkey_enable().values().contains(&app.get_hotkey_selected().to_string().to_string()){
                    ui.separator();
                    delete_shortcut_button(app, ui);
                }
            }
        );
    }
}
//PAINTING ON IMAGE PAGE
pub fn editing_page(ui: &mut Ui, app: &mut MyApp, frame: &mut eframe::Frame, ctx: &egui::Context){
    control_mouse_input(app, ctx);
    ui.horizontal(
        |ui|{
            back_button(app, ui, ctx);
            painting_combobox(app, ui);
            color_picker_combobox(app, ui);
            ui.separator();
            erase_button(app, ui, ctx);
            if ui.add(egui::ImageButton::new(
                app.get_icon(12).texture_id(ctx), Vec2::new(app.get_size_button(),app.get_size_button())
            )).clicked(){
                app.editing = None;
                frame.request_screenshot();
            }
        }
    );
    ui.separator();
    image_on_screen(app, frame, ui);
    draw_all_paintings(app, ui);
}
//DEFAULT LOCATION PAGE
pub fn saving_option_page(ui: &mut Ui, app: &mut MyApp, ctx: &egui::Context){
    save_folder_button(app, ui);
    ui.add_space(30.0);
    draw_text_edit(app, ui);
    ui.add_space(50.0);
    ok_default_save_button(app, ui, ctx);
}

}