pub(crate) mod draw_utils{
    use std::ops::Deref;
    use std::path::Path;
    use arboard::{Clipboard};
    use eframe::emath::Vec2;
    use egui::{Color32, emath, Pos2, Rounding, Stroke, Ui};
    use native_dialog::FileDialog;
    use crate::enums::app_enums::{EditType, HotkeysFunctions, RequestState, ScreenshotType};
    use crate::app::app_utils::MyApp;
    use crate::utils::utils::{get_possible_hotkeys_functions, keys_string, sort_key_modifier, sort_keys};

    ///DRAW NEW SCREENSHOT BUTTON
    pub fn draw_new_button( app: &mut MyApp, frame:&mut eframe::Frame, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(30.0);

        if ui.add(egui::Button::image_and_text(
            app.get_icon(0).texture_id(ctx),
            Vec2::new(30.0, 30.0),
            "NEW"
        )).clicked(){
            if app.get_request_state()==RequestState::Initialized {
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
                    ScreenshotType::CUSTOM,
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

    ///DRAW MONITOR CHOICE BUTTON
    pub fn draw_monitor_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context, number: usize, frame: &mut eframe::Frame){
        if ui.add(egui::Button::image_and_text(
            app.get_icon(1).texture_id(ctx),
            Vec2::new(30.0, 30.0),
            format!("MONITOR {}", number.to_string())
        )).clicked(){
            app.set_screen_selected(number-1);
            app.screen_request_init(frame);
        }
    }

    ///DRAW EDIT BUTTON
    pub fn draw_edit_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(20.0);
        //new button
        if ui.add(egui::Button::image_and_text(
            app.get_icon(9).texture_id(ctx),
            Vec2::new(30.0, 30.0),
            "EDIT"
        )).clicked(){
            app.set_first_edit_image();
            app.set_request_state(RequestState::EditImage)
        }
    }
    ///DRAW TEXT BUTTON
    pub fn draw_text_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(20.0);
        //new button
        if ui.add(egui::Button::image_and_text(
            app.get_icon(11).texture_id(ctx),
            Vec2::new(30.0, 30.0),
            "TEXT"
        )).clicked(){
            app.set_edit_type(EditType::Text)
        }
    }
    ///DRAW PAINTING BUTTON
    pub fn draw_paint_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(20.0);
        //new button
        if ui.add(egui::Button::image_and_text(
            app.get_icon(10).texture_id(ctx),
            Vec2::new(30.0, 30.0),
            "PAINT"
        )).clicked(){
            app.set_edit_type(EditType::Painting);
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
            //check if it's set default location and name
            let mut default_name : String = String::new();
            let mut location = String::new();
            if app.get_saved_data().get_location().is_some(){
                location = app.get_saved_data().get_location().unwrap();
            };
            if !app.get_saved_data().get_name().is_empty(){
                default_name = format!("{} {}",app.get_saved_data().get_name(), app.get_saved_data().get_screenshot_numbers())
            }
            let path = FileDialog::new()
                .add_filter("PNG Image", &["png"])
                .add_filter("JPEG image",&["jpg", "jpeg"] )
                .add_filter("Gif image", &["gif"])
                .add_filter("WebP image", &["webp"])
                .add_filter("Tiff image", &["tiff"])
                .set_filename(default_name.as_str())
                .set_location(Path::new(location.as_str().clone()))
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
            if !app.get_request_state().equal("EditImage"){
                if app.get_screen_type()==ScreenshotType::FULL || app.is_rect_choosen(){
                    ui.vertical_centered(
                        |ui|{
                            app.ui_with_image(frame,ui);
                        });
                }else{
                    app.ui_with_image(frame, ui);
                }
            }else{
                app.show_image(ui);
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
                app.set_request_state(RequestState::Processed); //transition to final state
            }
        }
    }
    ///DRAW BACK BUTTON THAT CONTROL THE GO BACK FE FLOW
    pub fn draw_back_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(20.0);
        let mut size = 0.0;
        if !app.get_request_state().equal("EditImage"){
            size = 70.0;
        }else{
            size = 30.0;
        }
        if ui.add(
            egui::ImageButton::new(
                app.get_icon(4).texture_id(ctx),
                Vec2::new(size,size)
            )
        ).clicked(){
            if app.is_rect_shown(){
                app.set_rect_shown(false);
                app.set_rect_position(1, Pos2::new(0.0,0.0));
                app.set_rect_position(2, Pos2::new(0.0,0.0));
            }else{
                app.go_back();
            }

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
            Vec2::new(30.0,30.0),
            |ui| {
                if ui.add(egui::Button::image_and_text(
                    app.get_icon(8).texture_id(ctx),
                    Vec2::new(20.0, 20.0),
                    "Custom hotkeys"
                )).clicked(){
                    app.set_request_state(RequestState::HotkeyWindow);
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("save preferences...").clicked(){
                    app.set_request_state(RequestState::SavePreferences);
                    ui.close_menu();
                }
            });
    }

    ///DRAW SAVE FOLDER LOCATION
    pub fn draw_save_folder(app: &mut MyApp, ui: &mut Ui){
        ui.horizontal(
            |ui|{
                ui.label("choose default folder location");
                if ui.button("choose folder").clicked(){
                    let location = FileDialog::new().show_open_single_dir().unwrap();
                    if let Some(path) = location{
                        app.set_save_location(path.to_str().unwrap().to_string())
                    }
                }
            }
        );
    }

    ///DRAW TEXT EDIT
    pub fn draw_text_edit(app: &mut MyApp, ui: &mut Ui){
        ui.horizontal(
            |ui|{
                ui.label("default saving name: ");
                ui.add(egui::TextEdit::singleline(&mut app.saved_data.save_name).hint_text("write the default name"))
            }
        );
    }

    ///LISTA DI TUTTE LE SHORTCUT CHE HAI ATTIVATO
    pub fn draw_enable_hotkeys_shortcuts(app: &mut MyApp, ui: &mut Ui){
        if !app.get_hotkey_enable().is_empty(){
            ui.label("LE TUE SHORTCUTS PREMI PER MODIFICARLE :");
            ui.separator();
            ui.add_space(15.0);
                for (mut k,v) in app.get_hotkey_enable(){
                    k.sort_by(sort_key_modifier);
                    let keys = keys_string(k);
                    if app.get_request_state().equal("HotkeysAdd"){
                        ui.add_enabled(false, egui::Button::new(format!("{function}: {keys}", function = v, keys = keys )));
                    }else{
                        if ui.button(format!("{function}: {keys}", function = v, keys = keys )).clicked(){
                            app.set_hotkey_selected(HotkeysFunctions::into_enum(v.as_str()));
                            app.set_request_state(RequestState::HotkeysSelection);
                        }
                    }
                    ui.separator();
                    ui.add_space(15.0);
                }
        }
    }

    ///ALL CUSTOM HOTKEYS FUNCTIONALITY COMBOBOX
    pub fn draw_add_hotkey_combobox(app: &mut MyApp, ui: &mut Ui){
        let all_selectable_functions = get_possible_hotkeys_functions(app.get_hotkey_enable());
        if !all_selectable_functions.contains(&app.hotkey_selected){
            app.set_hotkey_selected(all_selectable_functions[0]);
        }
        ui.add_space(20.0);
        egui::ComboBox::new("+", "")
            .selected_text(format!("{:?}", app.get_hotkey_selected().to_string()))
            .show_ui(ui, |ui|{
                for function in all_selectable_functions {
                    ui.selectable_value(&mut app.hotkey_selected, function, format!("{:?}", function.to_string()));
                }
            });
    }

    ///SELECT BUTTON IN SHORTCUT SELECTION
    pub fn draw_select_hotkey(app: &mut MyApp, ui: &mut Ui){
        if ui.button("SELECT").clicked(){
            app.set_request_state(RequestState::HotkeysSelection);
        }
    }
    ///OK BUTTON IN SHORTCUT SELECTION
    pub fn draw_ok_shortcut_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("OK").clicked(){
            app.set_hotkey_enable(app.get_hotkey_selected(), sort_keys(app.get_keys()));
            app.clear_keys();
            app.set_repeated_keys(false);
            app.set_request_state(RequestState::HotkeyWindow);
        }
    }

    ///DRAW OK SAVE DEFAULT BUTTON
    pub fn ok_default_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("OK").clicked(){
            if app.image_raw.is_some() || app.is_rect_choosen() {
                app.set_request_state(RequestState::Processed)
            }else{
                app.set_request_state(RequestState::Initialized)
            }
        }
    }

    ///DROP SHORTCUT IF WRONG
    pub fn draw_delete_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("RESET").clicked(){
            app.clear_keys();
            app.set_repeated_keys(false);
        }
    }

    ///DISABLE HOTKEY FUNCTIONS
    pub fn draw_delete_function_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("DELETE SHORTCUT").clicked(){
            app.clear_keys();
            app.remove_from_map_by_value(app.get_hotkey_selected());
            app.set_request_state(RequestState::HotkeyWindow);
        }
    }

    ///GO BACK FROM SHORTCUT MENU IN DIFFERENT STATES
    pub fn draw_back_menu_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("BACK").clicked(){
            if app.get_request_state().equal("HotkeyWindow"){
                app.set_request_state(RequestState::Initialized);
            }else if app.get_request_state().equal("HotkeysSelection"){
                app.clear_keys();
                app.set_request_state(RequestState::HotkeyWindow);
            }else if app.get_request_state().equal("HotkeysAdd"){
                app.set_request_state(RequestState::HotkeyWindow);
            }

        }
    }

    ///SCELTA DELLA SHORTCUT
    pub fn draw_shortcut_selection(app: &mut MyApp, ui: &mut Ui){
    let keys = keys_string(app.get_keys());

        ui.vertical_centered(|ui| {
            ui.label(format!("PREMI I TASTI PER LA SHORTCUT: {function}", function = app.get_hotkey_selected().to_string()));
        });
        ui.separator();
        ui.add_space(20.0);
        ui.label(
            format!("HAI SELEZIONATO: {keys}", keys= keys)
        );
    }

}
