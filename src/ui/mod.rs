extern crate gtk;

mod glade;

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

struct UiState {
    state: State
}

impl UiState {
    fn new() -> UiState {
        UiState {
            state: State::Initial
        }
    }

    fn transition_to(&mut self, cmd: KeyCommand) -> State {
        self.state = State::next(&self.state, &cmd);
        self.state
    }

    fn set_state(&mut self, state: State) {
        self.state = state;
    }
}

lazy_static! {
    static ref UI_STATE: Mutex<UiState> = Mutex::new(UiState::new());
}

fn current_state() -> State {
    match UI_STATE.lock() {
        Ok(inner) => inner.state,
        Err(e) => {
            println!("Couldn't acquire UI_STATE lock {:?}", e);
            State::Error        
        }
    }
}

pub fn build(tx: Sender<Command>, rx: Receiver<State>) {

    let factory = GladeObjectFactory::new();

    let file_path_entry: Entry = factory.get("filePathEntry1");
    let state_label = factory.get::<gtk::Label>("stateLabel");
    //let state_labelx = Arc::new(state_label.clone());

    gtk::timeout_add(100, move || {
        //Render anything in the render queue
        while let Ok(state) = rx.try_recv() {
            println!("Received state in ui updater thread {:?}", state);
            UI_STATE.lock().unwrap().set_state(state);  
            render(&state_label, current_state()); 
        }
        //So this will get invoked on every timeout interval, but the main thread does that anyway
        Continue(true)
    });
    
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
    let (load_button, load_button_tx) = get_obj::<Button>(&factory, "playButton", shared_tx.clone());
    let (mute_button, mute_button_tx) = get_obj::<Button>(&factory, "muteButton", shared_tx.clone());
    let (stop_button, stop_button_tx) = get_obj::<Button>(&factory, "stopButton", shared_tx.clone());

    let open_menu_item = factory.get::<gtk::MenuItem>("openMenuItem");
    
    let state_label = factory.get::<gtk::Label>("stateLabel");
    render(&state_label, current_state());

    let file_path_entry1 = file_path_entry.clone();
    open_menu_item.connect_activate(move |_| popup_file_chooser(&file_path_entry1));

    //aboutdialog1

    let file_path_entry2 = file_path_entry.clone();
    file_chooser_button.connect_clicked(move |_| popup_file_chooser(&file_path_entry2));
    
    let state_label2 = state_label.clone();
    load_button.connect_clicked(move |_| {
        let path = file_path_entry.get_text();
        send(&load_button_tx, KeyCommand::Load, path);
        let new_state = UI_STATE.lock().unwrap().transition_to(KeyCommand::Load);
        render(&state_label2, new_state);
    });

    mute_button.connect_clicked(move |_| send(&mute_button_tx, KeyCommand::Mute, None));
    let state_label1 = state_label.clone();
    stop_button.connect_clicked(move |_| {
        send(&stop_button_tx, KeyCommand::Stop, None);
        UI_STATE.lock().unwrap().transition_to(KeyCommand::Stop);
        render(&state_label1, current_state());
    });
    
    window.show_all();    
}

fn render(state_label: &gtk::Label, state_arg: State) {
    state_label.set_text(format!("{}", state_arg).as_str());
}

fn popup_file_chooser(file_path_entry: &Entry) {
    let dialog = build_file_chooser();  
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

fn send(tx: &Sender<Command>, key: KeyCommand, state_arg: Option<String>) {
    let cmd = Command::new_with_state(key, state_arg.unwrap_or_default());
    tx.send(cmd).map_err(|e| println!("Failed to send {:?}", e)).ok();
}

fn get_obj<T: gtk::IsA<gtk::Object>>(factory: &GladeObjectFactory, name: &'static str, tx: Rc<Sender<Command>>) -> (T, Rc<Sender<Command>>) {
    return (factory.get(name), tx);
}
