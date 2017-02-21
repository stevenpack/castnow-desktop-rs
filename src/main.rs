extern crate gtk;

mod launcher;
mod castnow;
mod ui;


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    ui::build();
    gtk::main();
}
