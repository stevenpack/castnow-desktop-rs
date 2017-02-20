extern crate gtk;

mod launcher;
mod castnow;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let play_button = Button::new_with_label("Play");
    v_box.pack_start(&play_button, false, false, 0);
    
    let pause_button = Button::new_with_label("Pause");
    v_box.pack_start(&pause_button, false, false, 0);

    window.add(&v_box);   
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let launcher = launcher::Launcher::new();
    let shared =  Rc::new(RefCell::new(launcher));

    let s1 = shared.clone();
     pause_button.connect_clicked(move |_| {
        println!("Clicked!");
        s1.borrow_mut().pause();
    });

    let s2 = shared.clone();
    play_button.connect_clicked(move |_| {
        println!("Clicked!");
        s2.borrow_mut().launch();
    });

   
    gtk::main();
}
