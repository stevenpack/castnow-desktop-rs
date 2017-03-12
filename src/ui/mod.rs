extern crate gtk;

mod glade;
mod handlers;
mod widgets;

use std::cell::{RefCell};
use gtk::prelude::*;
use gtk::{Window, WindowType};
use gtk::{Button, FileChooserDialog, Entry};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc,Mutex};
use std::rc::Rc;
use std::clone::Clone;
use std::thread;

use castnow::{KeyCommand, Command};
use state::State;
use self::glade::GladeObjectFactory;

pub struct AppModel {
    state: State,
    status: String,
    playing: bool,
    path: String,
    is_dirty: bool
}

pub struct AppState {
    channels: RefCell<AppChannels>,
    widgets: RefCell<AppWidgets>,
    model: RefCell<AppModel>
}

impl AppModel {
    fn new() -> AppModel {
        AppModel {
            state: State::Initial,
            status: "Ready".to_string(),
            playing: false,
            path: String::default(),
            is_dirty: false
        }
    }
}

impl AppState {
    pub fn new_rc() -> Rc<AppState> {
        Rc::new(AppState{
            channels: RefCell::new(AppChannels::new()),
            widgets: RefCell::new(AppWidgets::new()),
            model: RefCell::new(AppModel::new())
        })
    }

    pub fn init(&self, self_rc: Rc<Self>, rx: Receiver<State>, tx: Sender<Command>) {
        let mut channels = self_rc.channels.borrow_mut();
        channels.rx = Some(rx);
        channels.tx = Some(tx);

        self.build_widgets();
        self.attach_handlers(&self_rc);
        self.start_rendering_timer(&self_rc);   
    }

    fn attach_handlers(&self, self_rc: &Rc<Self>) {
        let widgets = self.widgets.borrow();        

        //todo: make method
        if let Some(ref popup_file_chooser_button) = widgets.popup_file_chooser_button {
            let self_clone = self_rc.clone();
            popup_file_chooser_button.connect_clicked(move |_| handlers::popup_file_chooser_button_clicked(&self_clone));
        }
        if let Some(ref play_button) = widgets.play_button {
            let self_clone = self_rc.clone();
            play_button.connect_clicked(move |_| handlers::play_clicked(&self_clone));
        }
        if let Some(ref stop_button) = widgets.stop_button {
            let self_clone = self_rc.clone();
            stop_button.connect_clicked(move |_| handlers::stop_clicked(&self_clone));
        }

        if let Some(ref window) = widgets.win {
            window.connect_delete_event(|_, _| {
                gtk::main_quit();
                Inhibit(false)
            });
            window.show_all();
        }
    }

    fn build_widgets(&self) {
        let mut widgets = self.widgets.borrow_mut();
        let factory = GladeObjectFactory::new();
        
        widgets.win = Some(factory.get("applicationwindow1"));
        widgets.file_path_entry = Some(factory.get("filePathEntry1"));
        widgets.popup_file_chooser_button = Some(factory.get("popupFileChooserButton"));
        widgets.play_button = Some(factory.get("playButton"));
        widgets.stop_button = Some(factory.get("stopButton"));
        widgets.state_label = Some(factory.get("stateLabel"));
        widgets.file_chooser_dialog = Some(widgets::build_file_chooser());
    }

    fn render_dirty(&self) {
        let widgets = self.widgets.borrow();
        let mut model = self.model.borrow_mut();
        if let Some(ref state_label) = widgets.state_label {
            state_label.set_text(model.status.as_str());
        }
        if let Some(ref file_path_entry) = widgets.file_path_entry {
            file_path_entry.set_text(model.path.as_str());
        }
        model.is_dirty = false;
    }

    fn start_rendering_timer(&self, self_rc: &Rc<Self>) {
        let self_clone = self_rc.clone();
        gtk::timeout_add(100, move || {
            //If the command thread queued some message, get it and update teh model
            if let Some(ref rx) = self_clone.channels.borrow().rx {
                while let Ok(state) = rx.try_recv() {
                    println!("Received state in ui thread {:?}", state);
                    //let's assume we also got some name/value pairs with enough info to locate our widget and render it                    
                    //Modifying the model and updating it must be separate scopes
                    let mut model = self_clone.model.borrow_mut();
                    model.state = state;
                    model.status = format!("{}", state);
                }
            }
            //println!("Checking...");
            //Update the UI to reflect any changes
            if self_clone.model.borrow().is_dirty {
                println!("Model dirty. Rendering...");
                self_clone.render_dirty();
            }             

            //So this will get invoked on every timeout interval, but the main thread does that anyway by
            //invoking idle
            Continue(true)
        });
    }
}

pub struct AppChannels {
    pub rx: Option<Receiver<State>>,
    pub tx: Option<Sender<Command>>
}

impl AppChannels {
    fn new() -> AppChannels {
        AppChannels {
            rx: None,
            tx: None
        }
    }
}

#[derive(Clone)]
pub struct AppWidgets {
    app:                    Option<gtk::Application>,    
    win:                    Option<gtk::Window>,
    file_path_entry:        Option<gtk::Entry>,
    popup_file_chooser_button: Option<gtk::Button>,
    play_button:            Option<gtk::Button>,
    stop_button:            Option<gtk::Button>,
    state_label:            Option<gtk::Label>,
    file_chooser_dialog:    Option<gtk::FileChooserDialog>
}

impl AppWidgets {
    pub fn new() -> AppWidgets {
        AppWidgets {
            app: None,
            win: None,
            file_path_entry: None,
            popup_file_chooser_button: None,
            play_button: None,
            stop_button: None,
            state_label: None,
            file_chooser_dialog: None
        }
    }
}
