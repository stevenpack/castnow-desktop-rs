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
    let (command_tx, command_rx) = channel::<castnow::Command>();
    //The Procssor sends back to new state to render
    let (state_tx, state_rx) = channel::<state::State>();

    let command_processor = command::Processor::new();    
    command_processor.start(command_rx, state_tx);
    let app = ui::AppState::new_rc();
    let app_rc = app.clone();
    app.init(app_rc, state_rx, command_tx);
    
    gtk::main();
}
