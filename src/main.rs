extern crate gtk;

mod launcher;
mod castnow;
mod ui;
mod command;

use std::sync::mpsc::channel;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;    
    }

    //UI sends commands commands to Processor running on a different thread to the UI
    let (tx, rx) = channel::<castnow::Command>();
    let command_processor = command::Processor::new();
    let ui = ui::UiBuilder::new();

    command_processor.start(rx);
    ui.build(tx);
    
    gtk::main();
}
