extern crate gtk;

use gtk::prelude::*;
use gtk::Builder;
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
        let file_chooser_dlg: FileChooserDialog = Self::get_obj(&builder, "filechooserdialog1");
        file_chooser_button.connect_clicked(move |_| {
            println!("{} clicked!", "Popup file dialog");
            file_chooser_dlg.show();    
        });

        let file_path_entry: Entry = Self::get_obj(&builder, "filePathEntry1");

        let load_button: Button = Self::get_obj(&builder, "playButton");
        load_button.connect_clicked(move |_| {
            let path = file_path_entry.get_text().unwrap_or_default();
            Self::send(&tx, KeyCommand::Load, path)});
        
        window.show_all();    
    }

    fn send(tx: &Sender<Command>, key: KeyCommand, state: String) {
        tx.send(Command::new_with_state(key, state)).map_err(|e| println!("Failed to send {:?}", e)).ok();
    }

    fn get_obj<T: gtk::IsA<gtk::Object>>(builder: &Builder, name: &'static str) -> T {
        let gtk_obj = builder.get_object(name);
        if gtk_obj.is_some() {
            return gtk_obj.unwrap();
        }
        panic!(format!("UI file corrupted. Unknown element {}", name));
    }
}