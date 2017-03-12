extern crate gtk;

use std::rc::Rc;
use castnow::{KeyCommand, Command};
use state::State;
use ui::AppState;
use gtk::prelude::*;

pub fn play_clicked(self_rc: &Rc<AppState>) {

    let mut model = self_rc.model.borrow_mut();
    model.state = State::Playing;
    model.status = "Playing...".to_string();
    model.is_dirty = true;

    //send a message to processor
    //it will send something back like Loaded
    let cmd = Command::new(KeyCommand::TogglePlayPause, model.state);    
    if let Some(ref tx) = self_rc.channels.borrow().tx {
        tx.send(cmd).ok();
    }       
}

pub fn stop_clicked(self_rc: &Rc<AppState>) {
    let mut model = self_rc.model.borrow_mut();
    model.state = State::Stopping;
    model.status = "Stopping...".to_string();
    model.is_dirty = true;

    let cmd = Command::new(KeyCommand::Stop, model.state);
    if let Some(ref tx) = self_rc.channels.borrow().tx {
        tx.send(cmd).ok();
    }       
}

pub fn popup_file_chooser_button_clicked(self_rc: &Rc<AppState>) {
    if let Some(ref dialog) = self_rc.widgets.borrow().file_chooser_dialog {
        if dialog.run() == gtk::ResponseType::Ok.into() {
            if let Some(path) = dialog.get_filename() {
                //This is possible to happen inside render? Needs to be a mutex...
                let mut model = self_rc.model.borrow_mut();
                let old_path = model.path.clone();
                println!("Path: {:?}", path);                
                path.to_str().map(|text| model.path = text.to_string());                                
                if model.path != old_path {
                    let cmd = Command::new_with_state(KeyCommand::Load, model.state, model.path.clone());
                    if let Some(ref tx) = self_rc.channels.borrow().tx {
                        tx.send(cmd).ok();
                    }
                }
                model.is_dirty = true;
            }
        }
        //dialog.destroy();  
        dialog.hide();
    }
}