pub(crate) mod draw_utils{
    use arboard::{Clipboard};
    use eframe::emath::Vec2;
    use egui::{Color32, emath, Rounding, Stroke, Ui};
    use native_dialog::FileDialog;
    use crate::enums::app_enums::{HotkeysFunctions, RequestState, ScreenshotType};
    use crate::app::app_utils::MyApp;
    use crate::utils::utils::{keys_string, sort_key_modifier};

    ///DRAW NEW SCREENSHOT BUTTON
    pub fn draw_new_button( app: &mut MyApp, frame:&mut eframe::Frame, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(30.0);
        //new button
        if ui.add(egui::Button::image_and_text(
            app.get_icon(0).texture_id(ctx),
            Vec2::new(30.0, 30.0),
            "NEW"
        )).clicked(){
            if app.get_request_state()==RequestState::INITIALIZED{
                app.screen_request_init(frame);
            }else{
                let screen_type = app.get_screen_type();
                app.reinit_app();
                app.set_erased(true);
                app.set_screen_type(screen_type);
                app.screen_request_init(frame);
            }

        }
    }

    ///DRAW MODE SELECTION COMBOBOX
    pub fn draw_combobox(app: &mut MyApp, ui: &mut Ui){
        ui.add_space(20.0);
        egui::ComboBox::new("mode", "")
            .selected_text(format!("{:?}", app.get_screen_type()))
            .show_ui(ui, |ui|{
                for screen_type in [
                    ScreenshotType::FULL,
                    ScreenshotType::RECT,
                ] {
                    ui.selectable_value(&mut app.screen_type, screen_type, format!("{:?}", screen_type));
                }
            });
    }

    ///DRAW ERASE BUTTON
    pub fn draw_erase_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(20.0);
        //new button
        if ui.add(egui::Button::image_and_text(
            app.get_icon(2).texture_id(ctx),
            Vec2::new(30.0, 30.0),
            "DISCARD"
        )).clicked(){
            app.go_back();
        }
    }

    ///DRAW SAVE FILE PICKER BUTTON
    //TODO: prova a vedere egui-file che forse è meno oneroso di rfd
    pub fn draw_file_picker(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(40.0);
        if ui.add(egui::Button::image_and_text(
            app.get_icon(5).texture_id(ctx),
            Vec2::new(30.0, 30.0),
            "Save as..."
        )).clicked(){
            let path = FileDialog::new()
                .add_filter("PNG Image", &["png"])
                .add_filter("JPEG image",&["jpg", "jpeg"] )
                .add_filter("Gif image", &["gif"])
                .add_filter("WebP image", &["webp"])
                .add_filter("Tiff image", &["tiff"])
                .show_save_single_file().unwrap();
            //save image
            if let Some(path) = path {
                app.get_image_raw().save(path.as_path()).expect("Error in saving image");
            }
        }
    }

    ///DRAW COPY BUTTON
    pub fn draw_copy_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context, clipboard: &mut Clipboard){
        if ui.add(egui::ImageButton::new(
            app.get_icon(6).texture_id(ctx),
            Vec2::new(30.0, 30.0)
        )).clicked(){
            app.copy_in_clipboard(clipboard);
        };
    }

    ///DRAW IMAGE ON UI DIFFERENTLY BASED ON USE CASE
    pub fn draw_image(app: &mut MyApp, frame: &mut eframe::Frame, ui:&mut Ui){
        if !app.is_erased(){
            if app.get_screen_type()==ScreenshotType::FULL || app.is_rect_choosen(){
                ui.vertical_centered(
                    |ui|{
                        app.ui_with_image(frame,ui);
                    });
            }else{
                app.ui_with_image(frame, ui);
            }
        }
    }

    /// OK BUTTON TO CONFIRM RECT SELECTION
    pub fn draw_ok_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(30.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(3).texture_id(ctx),
            Vec2::new(70.0,70.0)
        )).clicked(){
            if app.is_rect_shown(){
                app.replace_image_with_rect();
                app.set_rect_choosen(true);
                app.set_request_state(RequestState::PROCESSED); //transition to final state
            }
        }
    }
    ///DRAW BACK BUTTON THAT CONTROL THE GO BACK FE FLOW
    pub fn draw_back_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(20.0);
        if ui.add(
            egui::ImageButton::new(
                app.get_icon(4).texture_id(ctx),
                Vec2::new(70.0,70.0)
            )
        ).clicked(){
            app.go_back();
        }
    }

    ///DRAW RED SELECTED RECT
    pub fn draw_red_rect(app: &MyApp, ui: &mut Ui){
        let rect_position = app.get_rect_position();
        let points = vec![rect_position[0], rect_position[1]];
        ui.painter().rect(
            emath::Rect::from_points(&*points),
            Rounding::none(),
            Color32::from_rgba_unmultiplied(220, 220, 220, 9 as u8),
            Stroke::new(1.5,Color32::RED)
        );
    }

    ///DRAW MORE OPTION MENU
    pub fn draw_more_menu(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context) {
        ui.menu_image_button(
            app.get_icon(7).texture_id(ctx),
            Vec2::new(20.0,20.0),
            |ui| {
                if ui.add(egui::Button::image_and_text(
                    app.get_icon(8).texture_id(ctx),
                    Vec2::new(15.0, 15.0),
                    "Custom hotkeys"
                )).clicked(){
                    app.set_request_state(RequestState::HOTKEY_WINDOW);
                    ui.close_menu();
                }
            });
    }

    ///LISTA DI TUTTE LE SHORTCUT CHE HAI ATTIVATO
    pub fn draw_enable_hotkeys_shortcuts(app: &mut MyApp, ui: &mut Ui){
        if !app.get_hotkey_enable().is_empty(){
            ui.label("LE TUE SHORTCUTS : ");
            ui.add_space(30.0);
                for (mut k,v) in app.get_hotkey_enable(){
                    k.sort_by(sort_key_modifier);
                    let keys = keys_string(k);
                    ui.label(format!("{function}: {keys}", function = v, keys = keys ));
                }
        }
    }

    ///ALL CUSTOM HOTKEYS FUNCTIONALITY
    pub fn draw_add_hotkey_combobox(app: &mut MyApp, ui: &mut Ui){
        ui.add_space(20.0);
        egui::ComboBox::new("+", "")
            .selected_text(format!("{:?}", app.get_hotkey_selected().to_string()))
            .show_ui(ui, |ui|{
                for function in [
                    HotkeysFunctions::NewFull,
                    HotkeysFunctions::QuarterDownRight,
                    HotkeysFunctions::QuarterDownLeft,
                    HotkeysFunctions::QuarterTopLeft,
                    HotkeysFunctions::QuarterTopRight
                ] {
                    ui.selectable_value(&mut app.hotkey_selected, function, format!("{:?}", function.to_string()));
                }
            });
    }

    ///SELECT BUTTON IN SHORTCUT SELECTION
    pub fn draw_select_hotkey(app: &mut MyApp, ui: &mut Ui){
        if ui.button("SELECT").clicked(){
            app.set_request_state(RequestState::HOTKEYS_SELECTION);
        }
    }
    ///OK BUTTON IN SHORTCUT SELECTION
    pub fn draw_ok_shortcut_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("OK").clicked(){
            app.set_hotkey_enable(app.get_hotkey_selected(), app.get_keys());
            app.clear_keys();
            app.set_request_state(RequestState::HOTKEY_WINDOW);
        }
    }

    ///DROP SHORTCUT IF WRONG
    pub fn draw_delete_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("DELETE").clicked(){
            app.clear_keys();
        }
    }

    ///GO BACK FROM SHORTCUT MENU
    pub fn draw_back_menu_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("BACK").clicked(){
            app.set_request_state(RequestState::INITIALIZED);
        }
    }

    ///SCELTA DELLA SHORTCUT
    pub fn draw_shortcut_selection(app: &mut MyApp, ui: &mut Ui){
    let keys = keys_string(app.get_keys());
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.label("PREMI I TASTI PER SELEZIONARE LA SHORTCUT");
            })
        });
        ui.add_space(30.0);
        ui.label(
            format!("HAI SELEZIONATO: {keys}", keys= keys)
        );
    }

}
