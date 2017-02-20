extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType, Box};
use std::rc::Rc;
use std::cell::RefCell;
use castnow::{CastNow, KeyCommand};

type SharedCastNow = Rc<RefCell<CastNow>>;

pub fn build() {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("castnow desktop-rs");
    window.set_default_size(350, 70);
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let castnow = CastNow::new();
    let shared: SharedCastNow =  Rc::new(RefCell::new(castnow));

    add_button("Pause", shared.clone(), KeyCommand::Pause, &v_box);
    add_button("Mute", shared.clone(), KeyCommand::Mute, &v_box);
    add_button("Stop", shared.clone(), KeyCommand::Stop, &v_box);

    window.add(&v_box);   
    window.show_all();    
}

fn add_button(label: &'static str, shared_castnow: SharedCastNow, cmd: KeyCommand, gtk_box: &Box) {
    let button = Button::new_with_label(label);
     button.connect_clicked(move |_| {
        println!("{} clicked!", label);
        shared_castnow.borrow_mut().execute(&cmd);
    });
    gtk_box.pack_start(&button, false, false, 0)
}