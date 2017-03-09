extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType};

//most widgets are defined in glade. any defined in code are here

pub fn build_file_chooser() -> gtk::FileChooserDialog {
    let dialog = gtk::FileChooserDialog::new (
                Some("Choose DireFilectory"),
                Some(&Window::new(WindowType::Popup)),
                gtk::FileChooserAction::Open,
    );
    dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
    dialog.add_button("Select", gtk::ResponseType::Ok.into());
    dialog
}
