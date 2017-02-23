extern crate gtk;

use gtk::prelude::*;
use gtk::Builder;
use gtk::{Window, WindowType};
use gtk::{Button, FileChooserDialog, Entry};
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
        let window : gtk::Window = Self::get_obj(&builder, "applicationwindow1");  
        window.set_title("castnow desktop-rs");
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let file_chooser_button: Button = Self::get_obj(&builder, "popupFileChooserButton");
        let file_path_entry: Entry = Self::get_obj(&builder, "filePathEntry1");
        let load_button: Button = Self::get_obj(&builder, "playButton");
        let mute_button: Button = Self::get_obj(&builder, "muteButton");
        let stop_button: Button = Self::get_obj(&builder, "stopButton");

        let file_path_entry1 = file_path_entry.clone();
        file_chooser_button.connect_clicked(move |_| {
            println!("{} clicked!", "Popup file dialog");
            
            let dialog = FileChooserDialog::new (
                        Some("Choose DireFilectory"),
                        Some(&Window::new(WindowType::Popup)),
                        gtk::FileChooserAction::Open,
            );
            dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
            dialog.add_button("Select", gtk::ResponseType::Ok.into());

            if dialog.run() == gtk::ResponseType::Ok.into() {
                dialog.get_filename().map(|path| path.to_str().map(|text| file_path_entry1.set_text(text)));
            }
            dialog.destroy();   
        });

        let a = tx.clone();
        let b = tx.clone();
        let c = tx.clone();
        load_button.connect_clicked(move |_| {
            let path = file_path_entry.get_text();
            Self::send(&a, KeyCommand::Load, path)
        });

        mute_button.connect_clicked(move |_| Self::send(&b, KeyCommand::Mute, None));
        stop_button.connect_clicked(move |_| Self::send(&c, KeyCommand::Stop, None));
        
        window.show_all();    
    }

    fn send(tx: &Sender<Command>, key: KeyCommand, state: Option<String>) {
        tx.send(Command::new_with_state(key, state.unwrap_or_default())).map_err(|e| println!("Failed to send {:?}", e)).ok();
    }

    fn get_obj<T: gtk::IsA<gtk::Object>>(builder: &Builder, name: &'static str) -> T {
        let gtk_obj = builder.get_object(name);
        if gtk_obj.is_some() {
            return gtk_obj.unwrap();
        }
        panic!(format!("UI file corrupted. Unknown element {}", name));
    }
}