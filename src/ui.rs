extern crate gtk;

use gtk::prelude::*;
use gtk::{Button};
use gtk::Builder;
use std::sync::mpsc::Sender;
use castnow::{KeyCommand, Command};

pub struct UiBuilder {
}

impl UiBuilder {

    pub fn new() -> UiBuilder {
        UiBuilder{}
    }

    pub fn build(&self, tx: Sender<Command>) {

        // Load glade file  
        let glade_str = include_str!("ui.glade");
        let builder = Builder::new_from_string(glade_str);

        // Create Window   
        let window : gtk::Window = builder.get_object("applicationwindow1").unwrap();  
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        if let Some(popup_file_chooser_button) = UiBuilder::get_glade_object::<gtk::Button>(&builder, "popupFileChooserButton") {            
            popup_file_chooser_button.connect_clicked(move |_| {
                println!("{} clicked!", "Popup file dialog");
                if let Some(file_chooser_dlg) = UiBuilder::get_glade_object::<gtk::FileChooserDialog>(&builder, "filechooserdialog1") {
                    file_chooser_dlg.show();    
                    // shared1.execute(&KeyCommand::Mute);
                    // shared1.execute(&KeyCommand::Stop);
                }
            });
        }

        //let window = Window::new(WindowType::Toplevel);
        window.set_title("castnow desktop-rs");
        //window.set_default_size(350, 70);
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

        let load_button = Button::new_with_label("Load");
        load_button.connect_clicked(move |_| { tx.send(Command::new_with_state(KeyCommand::Load, "blah".to_string())).expect("Failed to send Load") });
        v_box.pack_start(&load_button, false, false, 0);

        window.add(&v_box);   
        window.show_all();    
    }

    // struct StateData {

    // }

    // impl StateData {
    //    fn on_connect_clicked(self, button: &gtk::Button) {

    //    } 
    // }

    fn get_glade_object<T: gtk::IsA<gtk::Object>>(builder: &Builder, name: &'static str) -> Option<T> {
        let gtk_obj: Option<T> = builder.get_object(name);
        if gtk_obj.is_some() {
            return gtk_obj;
        }
        println!("Failed to load object {:?} from glad file.", name);
        None
    }
}
// fn add_button(label: &'static str, shared_castnow: SharedCastNow, cmd: KeyCommand, gtk_box: &Box) {
//     let button = Button::new_with_label(label);
//     button.connect_clicked(move |_| {
//         //I don't think this needs to be borrow_mut any more, and therefore no Rc/RefCell?
//         println!("{} clicked!", label);
//         shared_castnow.borrow_mut().execute(&cmd);
//     });
//     gtk_box.pack_start(&button, false, false, 0)
// }}

