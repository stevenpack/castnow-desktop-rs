extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Box};
use gtk::Builder;
use std::rc::Rc;
use std::cell::RefCell;
use castnow::{CastNow, KeyCommand};

type SharedCastNow = Rc<RefCell<CastNow>>;

pub fn build() {

    // Load glade file  
    let glade_str = include_str!("ui.glade");
    let builder = Builder::new_from_string(glade_str);
    // Create Window   
    let window : gtk::Window = builder.get_object("applicationwindow1").unwrap();  
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Set window size  
    //window.set_default_size(400,400);  

    // Load components  
    // let button1 : gtk::Button = builder.get_object("button1").unwrap();  
    // let label1 : gtk::Label = builder.get_object("label1").unwrap();     

    //let window = Window::new(WindowType::Toplevel);
    window.set_title("castnow desktop-rs");
    //window.set_default_size(350, 70);
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let castnow = CastNow::new();
    let shared: SharedCastNow =  Rc::new(RefCell::new(castnow));

    let load_button = Button::new_with_label("Load");
    let shared_castnow = shared.clone();
    load_button.connect_clicked(move |_| {
        println!("{} clicked!", "Load");
        shared_castnow.borrow_mut().load("/home/steve/Downloads/SampleVideo_1280x720_5mb.mp".to_string());
    });
    v_box.pack_start(&load_button, false, false, 0);

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