extern crate gtk;

use gtk::prelude::*;
use gtk::Builder;
use gtk::{Window, WindowType};
use gtk::{Button, FileChooserDialog, Entry};
use std::sync::mpsc::Sender;
use std::rc::Rc;
use castnow::{KeyCommand, Command};

pub struct GladeObjectFactory {
    builder: Builder
}

impl GladeObjectFactory {
    pub fn new() -> GladeObjectFactory {
        // Load glade file  
        let glade_str = include_str!("ui.glade");
        let builder = Builder::new_from_string(glade_str);
        GladeObjectFactory {
            builder: builder
        }
    }

    pub fn get<T: gtk::IsA<gtk::Object>>(&self, name: &'static str) -> T {
        if let Some(gtk_obj) = self.builder.get_object(name) {
            return gtk_obj;
        }
        panic!(format!("UI file corrupted. Unknown element {}", name));
    }    
}

pub struct UiBuilder {
}

impl UiBuilder {

    pub fn new() -> UiBuilder {
        UiBuilder{}
    }

    pub fn build(&self, tx: Sender<Command>) {

        let factory = GladeObjectFactory::new();

        // Create Window   
        let window : gtk::Window = factory.get("applicationwindow1");
        window.set_title("castnow desktop-rs");
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let shared_tx = Rc::new(tx);

        let file_chooser_button: Button = factory.get("popupFileChooserButton");
        let file_path_entry: Entry = factory.get("filePathEntry1");
        let (load_button, load_button_tx) = Self::get_obj::<Button>(&factory, "playButton", shared_tx.clone());
        let (mute_button, mute_button_tx) = Self::get_obj::<Button>(&factory, "muteButton", shared_tx.clone());
        let (stop_button, stop_button_tx) = Self::get_obj::<Button>(&factory, "stopButton", shared_tx.clone());

        let open_menu_item = factory.get::<gtk::MenuItem>("openMenuItem");
        
        let file_path_entry1 = file_path_entry.clone();
        open_menu_item.connect_activate(move |_| Self::popup_file_chooser(&file_path_entry1));

        //aboutdialog1

        let file_path_entry2 = file_path_entry.clone();
        file_chooser_button.connect_clicked(move |_| Self::file_chooser_clicked(&file_path_entry2));
        
        load_button.connect_clicked(move |_| {
            let path = file_path_entry.get_text();
            Self::send(&load_button_tx, KeyCommand::Load, path)
        });

        mute_button.connect_clicked(move |_| Self::send(&mute_button_tx, KeyCommand::Mute, None));
        stop_button.connect_clicked(move |_| Self::send(&stop_button_tx, KeyCommand::Stop, None));
        
        window.show_all();    
    }

    fn file_chooser_clicked(file_path_entry: &Entry) {     
       Self::popup_file_chooser(&file_path_entry);
    }

    fn popup_file_chooser(file_path_entry: &Entry) {
        let dialog = Self::build_file_chooser();  
        if dialog.run() == gtk::ResponseType::Ok.into() {
            dialog.get_filename().map(|path| path.to_str().map(|text| file_path_entry.set_text(text)));
        }
        dialog.destroy();  
    }

    fn build_file_chooser() -> gtk::FileChooserDialog {
       let dialog = FileChooserDialog::new (
                    Some("Choose DireFilectory"),
                    Some(&Window::new(WindowType::Popup)),
                    gtk::FileChooserAction::Open,
       );
       dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
       dialog.add_button("Select", gtk::ResponseType::Ok.into());

       dialog
    }

    fn send(tx: &Sender<Command>, key: KeyCommand, state: Option<String>) {
        let cmd = Command::new_with_state(key, state.unwrap_or_default());
        tx.send(cmd).map_err(|e| println!("Failed to send {:?}", e)).ok();
    }

    fn get_obj<T: gtk::IsA<gtk::Object>>(factory: &GladeObjectFactory, name: &'static str, tx: Rc<Sender<Command>>) -> (T, Rc<Sender<Command>>) {
        return (factory.get(name), tx);
    }

}
