pub(crate) mod input{
    use std::cmp::{max, min};
    use arboard::Clipboard;
    use egui::Event::Key;
    use egui::{Pos2};
    use crate::enums::app_enums::{EditType, HotkeysFunctions, RectEdit};
    use crate::app::app_utils::MyApp;
    use crate::utils::utils::{change_rect, compare_keys, find_modifier};


    ///method to control mouse
    pub fn control_mouse_input(app: &mut MyApp, ctx: & egui::Context){
        ctx.input(
            |i|{
                //add position to draw
                if app.get_request_state().equal("EditImage"){
                    if i.pointer.hover_pos().is_some(){
                        if i.pointer.hover_pos().unwrap().y>54.0{
                            if i.pointer.primary_pressed(){
                                if let Some(editing) = app.get_editing().clone(){
                                    if i.pointer.press_origin().is_some(){
                                        let position = i.pointer.press_origin().unwrap().clone();
                                        match editing{
                                            EditType::Free => {app.push_new_line()}
                                            EditType::Square => {app.set_new_rect_position([position, position])}
                                            EditType::Circle => {app.set_new_circle_position([position, position])}
                                            EditType::Arrow => {app.set_new_arrow_position([position, position])}
                                            EditType::Highlight =>{app.set_new_highlight_position([position, position])}
                                            _ =>{}
                                        }
                                    }
                                }
                            }
                            if i.pointer.is_decidedly_dragging(){
                                if let Some(editing) = app.get_editing().clone(){
                                    if i.pointer.interact_pos().is_some(){
                                        if i.pointer.hover_pos().unwrap().y>54.0{
                                            let position = i.pointer.interact_pos().unwrap().clone();
                                            match editing{
                                                EditType::Free => {app.push_new_position(position)}
                                                EditType::Square => {app.update_rect_position(position)}
                                                EditType::Circle => {app.update_circle_position(position)}
                                                EditType::Arrow => {app.update_arrow_position(position)}
                                                EditType::Highlight =>{app.update_highlight_position(position)}
                                                _ =>{}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if !(i.pointer.hover_pos().is_some() && i.pointer.hover_pos().unwrap().x>((app.get_image_raw().width() as f32*0.92) + 12.0)){
                    app.set_outside_rect(false);
                    if !app.is_rect_shown(){
                        if i.pointer.primary_pressed(){
                            let origin = i.pointer.press_origin().unwrap();
                            app.set_rect_position(1,origin);
                            app.set_rect_position(2,origin);
                        }
                        if i.pointer.is_decidedly_dragging(){
                            if i.pointer.primary_down(){
                                if i.pointer.interact_pos().is_some(){
                                    let pos= i.pointer.interact_pos().unwrap();
                                    app.set_rect_position(2, pos);
                                }
                            }else{
                                let positions = app.get_rect_position();
                                if positions[0] != positions[1]{
                                    app.set_rect_shown(true);
                                }
                            }
                        }
                    }else{
                        //dragging feature
                        let min_x = min(app.get_rect_position()[0].clone().x as i32, app.get_rect_position()[1].clone().x as i32);
                        let min_y = min(app.get_rect_position()[0].clone().y as i32, app.get_rect_position()[1].clone().y as i32);
                        let max_x = max(app.get_rect_position()[0].clone().x as i32, app.get_rect_position()[1].clone().x as i32);
                        let max_y = max(app.get_rect_position()[0].clone().y as i32, app.get_rect_position()[1].clone().y as i32);
                        let mut mouse_pos: Pos2 = Pos2::default();
                        if i.pointer.hover_pos().is_some(){mouse_pos = i.pointer.hover_pos().unwrap()}
                        if i.pointer.primary_down(){
                            if let Some(edit) = app.get_rect_edit(){
                                match edit{
                                    RectEdit::HorizontalLeft => {
                                        if mouse_pos.y>min_y as f32 && mouse_pos.y<max_y as f32{
                                            change_rect(app,1,mouse_pos.x)
                                        }
                                    }
                                    RectEdit::HorizontalRight => {
                                        if mouse_pos.y>min_y as f32 && mouse_pos.y<max_y as f32{
                                            change_rect(app,2,mouse_pos.x)
                                        }
                                    }
                                    RectEdit::VerticalTop => {
                                        if mouse_pos.x>min_x as f32 && mouse_pos.x<max_x as f32{
                                            change_rect(app,4,mouse_pos.y)
                                        }
                                    }
                                    RectEdit::VerticalDown => {
                                        if mouse_pos.x>min_x as f32 && mouse_pos.x<max_x as f32{
                                            change_rect(app,3,mouse_pos.y)
                                        }
                                    }
                                }
                            }
                        }else{
                            //horizontal left
                            if (mouse_pos.x > min_x as f32 -30.0 && mouse_pos.x < min_x.clone() as f32 +30.0) && (mouse_pos.y>min_y as f32 && mouse_pos.y<max_y as f32){
                                app.set_rect_edit(Some(RectEdit::HorizontalLeft));
                            }
                            else if (mouse_pos.x > max_x as f32-30.0 && mouse_pos.x <max_x.clone() as f32+30.0) && (mouse_pos.y>min_y.clone() as f32 && mouse_pos.y<max_y.clone() as f32) {
                                app.set_rect_edit(Some(RectEdit::HorizontalRight));
                            }
                            //vertical top
                            else if (mouse_pos.y > min_y.clone() as f32 -30.0 && mouse_pos.y < min_y.clone() as f32 +30.0) && (mouse_pos.x>min_x.clone() as f32 && mouse_pos.x<max_x.clone() as f32){
                                app.set_rect_edit(Some(RectEdit::VerticalTop));
                            }
                            else if (mouse_pos.y > max_y.clone() as f32 -30.0 as f32 && mouse_pos.y < max_y.clone() as f32 +30.0) && (mouse_pos.x>min_x.clone() as f32 && mouse_pos.x<max_x.clone() as f32){
                                app.set_rect_edit(Some(RectEdit::VerticalDown));
                            }
                            // other positions
                            else{
                                app.set_rect_edit(None);
                            }
                        }

                    }
                }else{
                    app.set_outside_rect(true);
                }
            }
        );
    }
    ///method to control keyboard
    pub fn control_keyboard(app: &mut MyApp, ctx: & egui::Context, frame: &mut eframe::Frame, clipboard: &mut Clipboard){
        ctx.input(
            |i|{
                //COPY TO THE CLIPBOARD IF TERMINAL STATE AND PRESS 'COPY PATTERN'
                if i.events.contains(&egui::Event::Copy){
                    if app.get_request_state().equal("PROCESSED"){
                        //copy image in clipboard
                        app.copy_in_clipboard(clipboard); 
                    }
                }else{
                    //LISTEN HOTKEYS SHORTCUTS
                    i.events.iter().for_each(
                        |event|{
                            match event {
                                //listening "key events" 
                                Key {key, pressed, repeat, modifiers}=>{
                                    if !repeat.clone() && pressed.clone() {
                                        //pressing new combination from random part of the app 
                                        app.set_repeated_keys(false);
                                        let pressed_keys = if let Some(modifier) = find_modifier(modifiers){
                                            //modifier.iter().for_each(|modifier| app.set_pressed_key(modifier));
                                            let mut modifier_keys = modifier.iter().map(|el| el.to_string()).collect::<Vec<_>>().clone();
                                            modifier_keys.push(key.name().to_string());
                                            modifier_keys
                                        }else{
                                            vec![key.name().to_string()]
                                        };
                                        //set pressed key
                                        app.set_pressed_key(pressed_keys);
                                    } else if !pressed.clone() && !app.get_request_state().equal("HotkeysSelection") {
                                        //if i press a combination and i'm not in hotkeysSelection it could start a hotkey function 
                                        if !app.get_hotkey_enable().is_empty() {
                                            for (k, v) in app.get_hotkey_enable() {
                                                if compare_keys(app.get_press_keys(), k.clone()){
                                                    app.do_hotkey_function(HotkeysFunctions::into_enum(v.as_str()), frame);
                                                }
                                            }
                                        }
                                        app.clear_press_keys();
                                    }else if !pressed.clone() && app.get_request_state().equal("HotkeysSelection"){
                                        //setting particular hotkeys
                                        if !app.get_hotkey_enable().is_empty(){
                                            for k in app.get_hotkey_enable().keys(){
                                                if compare_keys(app.get_press_keys(), k.clone()){
                                                    app.set_repeated_keys(true);
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                )
                }
            }
        )
    }
}