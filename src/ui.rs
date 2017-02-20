extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};
use std::rc::Rc;
use std::cell::RefCell;
use castnow::KeyCommand;
use launcher::Launcher;

pub fn build() {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let play_button = Button::new_with_label("Play");
    v_box.pack_start(&play_button, false, false, 0);
    
    let pause_button = Button::new_with_label("Pause");
    v_box.pack_start(&pause_button, false, false, 0);

     let mute_button = Button::new_with_label("Mute");
    v_box.pack_start(&mute_button, false, false, 0);

    window.add(&v_box);   
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let launcher = Launcher::new();
    let shared =  Rc::new(RefCell::new(launcher));

    let s1 = shared.clone();
     pause_button.connect_clicked(move |_| {
        println!("Pause clicked!");
        s1.borrow_mut().execute(KeyCommand::Pause);
    });

    let s3 = shared.clone();
     mute_button.connect_clicked(move |_| {
        println!("Mute clicked!");
        s3.borrow_mut().execute(KeyCommand::Mute);
    });

    let s2 = shared.clone();
    play_button.connect_clicked(move |_| {
        println!("Clicked!");
        s2.borrow_mut().launch();
    });

}