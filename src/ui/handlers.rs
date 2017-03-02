use std::rc::Rc;
use castnow::{KeyCommand, Command};
use ui::AppState;

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