#[macro_use]
extern crate lazy_static;
extern crate gtk;

mod shell;
mod castnow;
mod ui;
mod command;
mod state;

use std::sync::mpsc::channel;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;    
    }

    //UI sends commands commands to Processor running on a different thread to the UI
    let (tx1, rx1) = channel::<castnow::Command>();
    let (tx2, rx2) = channel::<state::State>();

    let command_processor = command::Processor::new();    
    command_processor.start(rx1, tx2);
    //ui::build(tx1, rx2);
    let app = ui::AppState::new_rc();
    let app_rc = app.clone();
    app.init(app_rc, rx2, tx1);
    
    gtk::main();
}
