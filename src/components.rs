pub(crate) mod components {
    use std::path::Path;
    use arboard::{Clipboard};
    use eframe::emath::Vec2;
    use egui::{Color32, emath, Pos2, Rect, Rounding, Stroke, Ui};
    use native_dialog::FileDialog;
    use std::string::String;
    use crate::enums::app_enums::{EditType, HotkeysFunctions, RequestState, SizeType};
    use crate::app::app_utils::MyApp;
    use crate::command::command::{choice_monitor_command, delete_shortcut_command, draw_image_command, go_back_command, new_screen_command, ok_save_location_command, reset_shortcut_command, save_image_command, select_screen_command, set_shortcut_command};
    use crate::utils::utils::{get_color_from_str, get_possible_hotkeys_functions, get_str_from_color, keys_string};

    ///NEW SCREENSHOT BUTTON
    pub fn new_button(app: &mut MyApp, frame:&mut eframe::Frame, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(20.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(0).texture_id(ctx),
            Vec2::new(app.get_size_button(), app.get_size_button())
        )).on_hover_text("NEW SCREENSHOT").clicked(){
            new_screen_command(app,frame);
        }
    }
    ///DRAW BACK BUTTON THAT CONTROL THE GO BACK FE FLOW
    pub fn back_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(20.0);
        let size = if !app.get_request_state().equal("ChoiceScreen"){30.0}else{70.0};
        if ui.add(
            egui::ImageButton::new(
                app.get_icon(4).texture_id(ctx),
                Vec2::new(size,size)
            )
        ).on_hover_text("GO BACK").clicked(){
            go_back_command(app);
        }
    }
    ///MONITOR CHOICE BUTTON
    pub fn monitor_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context, number: usize, frame: &mut eframe::Frame){
        ui.vertical_centered(|ui|{
            if ui.add(egui::Button::image_and_text(
                app.get_icon(1).texture_id(ctx),
                Vec2::new(70.0, 70.0),
                format!("MONITOR {}", number.clone().to_string())
            )).clicked(){
                choice_monitor_command(app, frame, number.clone());
            }
        });
    }
    ///EDIT BUTTON
    pub fn edit_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(10.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(9).texture_id(ctx),
            Vec2::new(30.0, 30.0)
        )).on_hover_text("PAINT ON IMAGE").clicked(){
            app.set_request_state(RequestState::EditImage)
        }
    }
    ///DRAW SAVE FILE PICKER BUTTON
    pub fn file_save_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context){
        ui.add_space(10.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(5).texture_id(ctx),
            Vec2::new(30.0, 30.0)
        )).on_hover_text("SAVE AS...").clicked(){
            save_image_command(app);
        }
    }
    ///COPY BUTTON
    pub fn copy_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context, clipboard: &mut Clipboard){
        ui.add_space(10.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(6).texture_id(ctx),
            Vec2::new(30.0, 30.0)
        )).on_hover_text("COPY").clicked(){
            app.copy_in_clipboard(clipboard);
        };
    }
    ///SELECT BUTTON IN SHORTCUT SELECTION
    pub fn select_hotkey_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("SELECT").clicked(){
            app.set_request_state(RequestState::HotkeysSelection);
        }
    }
    ///OK BUTTON IN SHORTCUT SELECTION
    pub fn ok_shortcut_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("OK").clicked(){
            set_shortcut_command(app);
        }
    }
    ///OK BUTTON TO CONFIRM RECT SELECTION
    pub fn screen_selected_button(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context, frame: &mut eframe::Frame){
        ui.add_space(30.0);
        if ui.add(egui::ImageButton::new(
            app.get_icon(3).texture_id(ctx),
            Vec2::new(70.0,70.0)
        )).clicked(){
            select_screen_command(app, frame);
        }
    }
    ///SAVE FOLDER LOCATION
    pub fn save_folder_button(app: &mut MyApp, ui: &mut Ui){
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
    ///RESET SHORTCUT BUTTON
    pub fn reset_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("RESET").clicked(){
            reset_shortcut_command(app);
        }
    }

    pub fn back_shortcut_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("BACK").clicked(){
            go_back_command(app);
        }
    }
    ///DELETE SHORTCUT BUTTON
    pub fn delete_shortcut_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("DELETE SHORTCUT").clicked(){
            delete_shortcut_command(app);
        }
    }
    ///OK SAVE DEFAULT BUTTON
    pub fn ok_default_save_button(app: &mut MyApp, ui: &mut Ui){
        if ui.button("OK").clicked(){
            ok_save_location_command(app);
        }
    }
    ///---------------------------------------------------------------------------------------------
    /// COMBOBOX

    ///DELAY COMBOBOX
    pub fn delay_combobox(app: &mut MyApp, ui: &mut Ui){
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
    ///PAINTING COMBOBOX
    pub fn painting_combobox(app: &mut MyApp, ui: &mut Ui){
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
    ///COLOR PICKER COMBOBOX
    pub fn color_picker_combobox(app: &mut MyApp, ui: &mut Ui){
        if app.get_editing().is_some(){
            if app.get_editing().unwrap().equal("Highlight"){
                egui::ComboBox::new("highlight_size", "")
                    .width(100.0)
                    .selected_text(format!("{:?}", app.get_highlight_size().unwrap()))
                    .show_ui(ui, |ui|{
                        for size in [SizeType::Small, SizeType::Medium, SizeType::Large]{
                            ui.selectable_value(&mut app.highlight_size, Some(size), format!("{:?}", size));
                        } });
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
    ///ALL CUSTOM HOTKEYS FUNCTIONALITY COMBOBOX
    pub fn add_hotkey_combobox(app: &mut MyApp, ui: &mut Ui){
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
    ///---------------------------------------------------------------------------------------------
    /// PAINTING

    ///RECT PAINTING EDIT
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
    ///ARROW PAINTING EDIT
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
    ///CIRCLE PAINTING EDIT
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
    ///HIGHLITER
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
    ///DRAW ALL PAINTINGS
    pub fn draw_all_paintings(app: &mut MyApp, ui: &mut Ui){
        draw_line(app, ui);
        draw_rect(app, ui);
        draw_cirlce(app, ui);
        draw_arrow(app, ui);
        draw_highlighter(app, ui);
    }

    ///DRAW IMAGE ON UI DIFFERENTLY BASED ON USE CASE
    pub fn image_on_screen(app: &mut MyApp, frame: &mut eframe::Frame, ui:&mut Ui){
        draw_image_command(app, frame, ui);
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
    ///MORE OPTION MENU
    pub fn more_menu(app: &mut MyApp, ui: &mut Ui, ctx: &egui::Context) {
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
    ///TEXT EDIT FOR DEFAULT NAME
    pub fn draw_text_edit(app: &mut MyApp, ui: &mut Ui){
        ui.horizontal(
            |ui|{
                ui.label("default saving name: ");
                ui.add(egui::TextEdit::singleline(&mut app.saved_default.save_name).hint_text("write the default name"))
            }
        );
    }
    ///LISTA DI TUTTE LE SHORTCUT CHE HAI ATTIVATO
    pub fn show_enable_hotkeys_shortcuts(app: &mut MyApp, ui: &mut Ui){
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
    ///SHORTCUT SELECTION
    pub fn shortcut_selection(app: &mut MyApp, ui: &mut Ui){
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
