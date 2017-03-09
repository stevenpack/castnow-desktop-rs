extern crate gtk;

use std::rc::Rc;
use castnow::{KeyCommand, Command};
use ui::AppState;
use gtk::prelude::*;
pub fn load_clicked(self_rc: &Rc<AppState>) {
    //send a message to processor
    let cmd = Command::new(KeyCommand::Load);
    if let Some(ref tx) = self_rc.channels.borrow().tx {
        tx.send(cmd).ok();
    }       
}

pub fn stop_clicked(self_rc: &Rc<AppState>) {
    //send a message to processor
    let cmd = Command::new(KeyCommand::Stop);
    if let Some(ref tx) = self_rc.channels.borrow().tx {
        tx.send(cmd).ok();
    }       
}

pub fn popup_file_chooser_button_clicked(self_rc: &Rc<AppState>) {
    if let Some(ref dialog) = self_rc.widgets.borrow().file_chooser_dialog {
        if dialog.run() == gtk::ResponseType::Ok.into() {
            //.map(|path| path.to_str().map(|text| file_path_entry.set_text(text)));
            if let Some(path) = dialog.get_filename() {
                println!("Path: {:?}", path);
                let mut model = self_rc.model.borrow_mut();
                path.to_str().map(|text| model.path = text.to_string());
                model.is_dirty = true;
            }
            
            
        }
        dialog.destroy();  
    }
}
