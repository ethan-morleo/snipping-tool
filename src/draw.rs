pub(crate) mod draw_utils{
    use std::path::Path;
    use arboard::{Clipboard};
    use eframe::emath::Vec2;
    use egui::{Color32, emath, Pos2, Rect, Rounding, Stroke, Ui};
    use native_dialog::FileDialog;
    use std::string::String;
    use crate::enums::app_enums::{EditType, HotkeysFunctions, RequestState, ScreenshotType, SizeType};
    use crate::app::app_utils::MyApp;
    use crate::utils::utils::{get_color_from_str, get_possible_hotkeys_functions, get_str_from_color, keys_string};

    ///DRAW NEW SCREENSHOT BUTTON
    pub fn draw_new_button( app: &mut MyApp, frame:&mut eframe::Frame, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(20.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(0).texture_id(ctx),
            Vec2::new(30.0, 30.0)
        )).on_hover_text("NEW SCREENSHOT").clicked(){
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
        ui.add_space(15.0);
        ui.label("AREA:");
        ui.add_space(1.0);
        egui::ComboBox::new("mode", "")
            .width(80.0)
            .selected_text(format!("{:?}", app.get_screen_type()))
            .show_ui(ui, |ui|{
                for screen_type in [
                    ScreenshotType::FULL,
                    ScreenshotType::CUSTOM
                ] {
                    ui.selectable_value(&mut app.screen_type, screen_type, format!("{:?}", screen_type));
                }
            });
    }

    ///DRAW ERASE BUTTON
    pub fn draw_erase_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(10.0);
        //new button
        if ui.add(egui::ImageButton::new(
            app.get_icon(2).texture_id(ctx),
            Vec2::new(30.0, 30.0)
        )).on_hover_text("ERASE").clicked(){
            if app.get_request_state().equal("EditImage"){
                app.erase_drawing();
            }else{
                app.go_back();
            }
        }
    }

    ///DRAW MONITOR CHOICE BUTTON
    pub fn draw_monitor_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context, number: usize, frame: &mut eframe::Frame){
        ui.vertical_centered(|ui|{
            if ui.add(egui::Button::image_and_text(
                app.get_icon(1).texture_id(ctx),
                Vec2::new(70.0, 70.0),
                format!("MONITOR {}", number.to_string())
            )).clicked(){
                app.set_screen_selected(number-1);
                app.screen_request_init(frame);
            }
        });
    }

    ///DRAW EDIT BUTTON
    pub fn draw_edit_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(10.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(9).texture_id(ctx),
            Vec2::new(30.0, 30.0)
        )).on_hover_text("PAINT ON IMAGE").clicked(){
            app.set_request_state(RequestState::EditImage)
        }
    }
    ///DRAW DELAY COMBOBOX
    pub fn draw_delay_combobox(app: &mut MyApp, ui: &mut Ui){
        ui.add_space(5.0);
        egui::ComboBox::new("delay", "")
            .width(40.0)
            .selected_text(format!("{:?}", app.get_delay()))
            .show_ui(ui, |ui|{
                for delay_type in [
                    0,
                    2,
                    4,
                    5
                ] {
                    ui.selectable_value(&mut app.delay, delay_type, format!("{:?}", delay_type));
                }
            });
    }
    ///DRAW PAINTING COMBOBOX
    pub fn draw_painting_combobox(app: &mut MyApp, ui: &mut Ui){
        let selected_edit = match app.get_editing(){
            None => {String::from("PAINTING TYPE")}
            Some(edit) => {
                let edit_string = edit.to_string().clone();
                if edit_string.eq("Text"){
                    String::from("PAINTING TYPE")
                }else{
                    edit_string
                }
            }
        };
        egui::ComboBox::new(
            "paint", "", )
            .width(150.0)
            .selected_text(selected_edit)
            .show_ui(ui, |ui|{
                for edit_type in [
                    EditType::Free,
                    EditType::Square,
                    EditType::Circle,
                    EditType::Arrow,
                    EditType::Highlight
                ] {
                    ui.selectable_value(&mut app.editing, Some(edit_type), edit_type.to_string());
                }
            });
    }

    ///DRAW COLOR PICKER BUTTON
    pub fn draw_color_picker_button(app: &mut MyApp, ui: &mut Ui){
        if app.get_editing().is_some(){
            if app.get_editing().unwrap().equal("Highlight"){
                egui::ComboBox::new("highlight_size", "")
                    .width(100.0)
                    .selected_text(format!("{:?}", app.get_highlight_size().unwrap()))
                    .show_ui(ui, |ui|{
                        for size in [SizeType::Small, SizeType::Medium, SizeType::Large]{
                            ui.selectable_value(&mut app.highlight_size, Some(size), format!("{:?}", size));
                        }
                    });
            }else{
                egui::ComboBox::new("color", "")
                    .width(100.0)
                    .selected_text(get_str_from_color(app.color))
                    .show_ui(ui, |ui|{
                        for color in [
                            "RED",
                            "BLUE",
                            "YELLOW",
                            "BLACK",
                            "BROWN",
                            "GREEN"
                        ]{
                            ui.selectable_value(&mut app.color, get_color_from_str(color), color);
                        }
                    });
            }
        }

    }

    ///DRAW RECT PAINTING EDIT
    pub fn draw_rect(app: &mut MyApp, ui: &mut Ui){
        for rect_points in app.get_rect_paint_position(){
            let points = vec![rect_points[0], rect_points[1]];
            ui.painter().rect_stroke(
                emath::Rect::from_points(&*points),
                Rounding::none(),
                Stroke::new(2.0,app.color)
            );
        }
    }
    ///DRAW ARROW PAINTING EDIT
    pub fn draw_arrow(app: &mut MyApp, ui: &mut Ui){
        for arrow_points in app.get_arrow_paint_position(){
            let vec2 = Vec2::new((arrow_points[1].x - arrow_points[0].x) as f32, (arrow_points[1].y - arrow_points[0].y) as f32);
            ui.painter().arrow(
                arrow_points[0],
                vec2,
                Stroke::new(2.0, app.color)
            )
        }
    }
    ///DRAW CIRCLE PAINTING EDIT
    pub fn draw_cirlce(app: &mut MyApp, ui: &mut Ui){
        for circle_points in app.get_circle_paint_position(){
            let radius = f32::sqrt(((circle_points[0].x - circle_points[1].x).powi(2)) + (circle_points[0].y - circle_points[1].y).powi(2));
            ui.painter().circle_stroke(
                circle_points[0],
                        radius,
                Stroke::new(2.0, app.color)
            );
        }
    }
    ///DRAW HIGHLITER
    pub fn draw_highlighter(app : &mut MyApp, ui: &mut Ui){
        let size = match app.get_highlight_size().unwrap() {
            SizeType::Small => {12.0}
            SizeType::Medium => {20.0}
            SizeType::Large => {35.0}
        };
        for highilight_points in app.get_highlight_paint_position(){
            let final_coords = highilight_points[0].y +size;
            let mut final_point = highilight_points[1];
            final_point.y = final_coords;
            ui.painter().rect_filled(
                Rect::from_two_pos(highilight_points[0], final_point),
                Rounding::none(),
                Color32::from_rgba_unmultiplied(255, 255, 0, 8u8)
            )
        }

    }
    ///DRAW SAVE FILE PICKER BUTTON
    //TODO: prova a vedere egui-file che forse è meno oneroso di rfd
    pub fn draw_file_picker(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(10.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(5).texture_id(ctx),
            Vec2::new(30.0, 30.0)
        )).on_hover_text("SAVE AS...").clicked(){
            //check if it's set default location and name
            let mut default_name : String = String::new();
            let mut location = String::new();
            if app.get_default().get_location().is_some(){
                location = app.get_default().get_location().unwrap();
            };
            if !app.get_default().get_name().is_empty(){
                default_name = format!("{} {}", app.get_default().get_name(), app.get_default().get_screenshot_numbers())
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

    ///DRAW ALL PAINTINGS
    pub fn draw_all_paintings(app: &mut MyApp, ui: &mut Ui){
        draw_line(app, ui);
        draw_rect(app, ui);
        draw_cirlce(app, ui);
        draw_arrow(app, ui);
        draw_highlighter(app, ui);
    }

    ///DRAW COPY BUTTON
    pub fn draw_copy_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context, clipboard: &mut Clipboard){
        ui.add_space(10.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(6).texture_id(ctx),
            Vec2::new(30.0, 30.0)
        )).on_hover_text("COPY").clicked(){
            app.copy_in_clipboard(clipboard);
        };
    }

    ///DRAW IMAGE ON UI DIFFERENTLY BASED ON USE CASE
    pub fn draw_image(app: &mut MyApp, frame: &mut eframe::Frame, ui:&mut Ui){
        if !app.is_erased(){
            if app.get_screen_type()==ScreenshotType::FULL || app.is_rect_choosen(){
                ui.horizontal_centered(
                    |ui|{
                        ui.vertical_centered(
                            |ui|{
                                app.ui_with_image(frame,ui);
                            });
                    }
                );
            }else{
                app.ui_with_image(frame, ui);
            }
        }
    }

    /// OK BUTTON TO CONFIRM RECT SELECTION
    pub fn draw_ok_button(app: &mut MyApp,ui: &mut Ui, ctx: &egui::Context, frame: &mut eframe::Frame){
        ui.add_space(30.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(3).texture_id(ctx),
            Vec2::new(70.0,70.0)
        )).clicked(){
            if app.is_rect_shown(){
                app.replace_image_with_rect();
                app.set_rect_choosen(true);
                let image_size = app.get_rect_image().size();
                frame.set_fullscreen(false);
                frame.set_maximized(false);
                frame.set_window_size(Vec2::new(image_size[0] as f32*2.0, image_size[1] as f32*2.0));
                frame.set_centered();
                app.set_request_state(RequestState::Processed); //transition to final state
            }
        }
    }
    ///DRAW BACK BUTTON THAT CONTROL THE GO BACK FE FLOW
    pub fn draw_back_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(20.0);
        let size = if app.get_request_state().equal("EditImage"){30.0}else{70.0};
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
            }else if app.get_request_state().equal("EditImage") {
                app.editing = None;
                app.erase_drawing();
                app.set_request_state(RequestState::Processed)
            } else{
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

    pub fn draw_line(app: &MyApp, ui: &mut Ui){
        if !app.get_painting_position().is_empty(){
            let lines = app.get_painting_position().clone();
            let shapes = lines
                .iter()
                .filter(|line| line.len() >= 2)
                .map(|line| {
                    let points: Vec<Pos2> = line.clone();
                    egui::Shape::line(points,
                    Stroke::new(2.0, app.color))
                });
            ui.painter().extend(shapes)
        }
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
                ui.add(egui::TextEdit::singleline(&mut app.saved_default.save_name).hint_text("write the default name"))
            }
        );
    }

    ///LISTA DI TUTTE LE SHORTCUT CHE HAI ATTIVATO
    pub fn draw_enable_hotkeys_shortcuts(app: &mut MyApp, ui: &mut Ui){
        if !app.get_hotkey_enable().is_empty(){

            ui.label("LE TUE SHORTCUTS PREMI PER MODIFICARLE");
            ui.separator();
            ui.add_space(15.0);
                for (k,v) in app.get_hotkey_enable(){
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
            app.set_hotkey_enable(app.get_hotkey_selected(), app.get_press_keys());
            app.clear_press_keys();
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
            app.clear_press_keys();
            app.set_repeated_keys(false);
        }
    }

    ///DISABLE HOTKEY FUNCTIONS
    pub fn draw_delete_function_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("DELETE SHORTCUT").clicked(){
            app.clear_press_keys();
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
                app.clear_press_keys();
                app.set_request_state(RequestState::HotkeyWindow);
            }else if app.get_request_state().equal("HotkeysAdd"){
                app.set_request_state(RequestState::HotkeyWindow);
            }
        }
    }

    ///SCELTA DELLA SHORTCUT
    pub fn draw_shortcut_selection(app: &mut MyApp, ui: &mut Ui){
    let keys = keys_string(app.get_press_keys());

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
