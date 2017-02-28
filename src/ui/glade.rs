extern crate gtk;

use gtk::Builder;


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
        panic!(format!("UI file corrupted. Unknown element of this type '{}'", name));
    }    
}
