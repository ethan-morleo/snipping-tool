pub(crate) mod input{
    use arboard::Clipboard;
    use egui::Event::Key;
    use itertools::Itertools;
    use crate::enums::app_enums::{HotkeysFunctions, KeysEnum};
    use crate::app::app_utils::MyApp;
    use crate::utils::utils::{find_modifier, set_keys_or_press_keys, sort_keys};

    ///method to control mouse
    pub fn control_mouse_input(app: &mut MyApp, ctx: & egui::Context){
        ctx.input(
            |i|{
                //ho iniziato a fare la diagonale del rettangolo
                //controllo che il mouse sia dentro l'immagine
                if !(i.pointer.hover_pos().is_some() && i.pointer.hover_pos().unwrap().x>((app.get_full_image().width() as f32*0.92) + 12.0)){
                    app.set_outside_rect(false);
                    if i.pointer.primary_pressed(){
                        app.set_rect_position(1,i.pointer.press_origin().unwrap());
                        app.set_rect_position(2,i.pointer.press_origin().unwrap());
                    }
                    if i.pointer.is_decidedly_dragging(){
                        if i.pointer.primary_down(){
                            app.set_rect_position(2, i.pointer.interact_pos().unwrap());
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
                if i.events.contains(&egui::Event::Copy) && app.get_request_state().equal("PROCESSED"){
                    //copy image in clipboard
                    app.copy_in_clipboard(clipboard);
                }

                //LISTEN SHORTCUTS
                i.events.iter().for_each(
                    |event|{
                        match event {
                            //ascolto la tastiera dagli eventi key
                            Key {key, pressed, repeat, modifiers}=>{
                                if !*repeat && *pressed {
                                    if let Some(modifier) = find_modifier(modifiers){
                                        modifier.iter().for_each(|modifier| set_keys_or_press_keys(app,app.get_request_state(),KeysEnum::Modifier(*modifier)));
                                    }
                                    set_keys_or_press_keys(app,app.get_request_state(),KeysEnum::Key(*key))
                                } else if !*pressed && !app.get_request_state().equal("HOTKEYS_SELECTION") {
                                    //cerco corrispondenza nella mappa se non è vuota
                                    if !app.get_hotkey_enable().is_empty() {
                                        for (k, v) in app.get_hotkey_enable() {
                                            let sort_pressed_keys = sort_keys(app.get_press_keys());
                                            let sort_hotkeys = sort_keys(k.clone());

                                            if sort_pressed_keys.iter().unique().collect::<Vec<_>>() == sort_hotkeys.iter().unique().collect::<Vec<_>>() {
                                                app.do_hotkey_function(HotkeysFunctions::into_enum(v.as_str()), frame);
                                            }
                                        }
                                    }
                                    app.clear_press_keys();
                                }
                            }
                            _ => {}
                        }
                    }
                )
            }
        )
    }
}