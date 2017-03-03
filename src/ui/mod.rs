extern crate gtk;

mod glade;
mod handlers;
mod ui_state;

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
use self::ui_state::{UiState, Channel};

lazy_static! {
    static ref UI_STATE: Mutex<UiState> = Mutex::new(UiState::new());
}

pub struct AppModel {
    status: String,
    playing: bool
}

pub struct AppState {
    channels: RefCell<AppChannels>,
    widgets: RefCell<AppWidgets>,
    model: RefCell<AppModel>
}

impl AppModel {
    fn new() -> AppModel {
        AppModel {
            status: "Ready".to_string(),
            playing: false
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
        if let Some(ref load_button) = widgets.load_button {
            let self_clone = self_rc.clone();
            load_button.connect_clicked(move |_| handlers::load_clicked(&self_clone));
        }
        if let Some(ref stop_button) = widgets.stop_button {
            let self_clone = self_rc.clone();
            stop_button.connect_clicked(move |_| handlers::load_clicked(&self_clone));
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
        //window.set_title("castnow desktop-rs");
        widgets.file_path_entry = Some(factory.get("filePathEntry1"));
        widgets.load_button = Some(factory.get("playButton"));
        widgets.state_label = Some(factory.get("stateLabel"));
    }

    fn render_dirty(&self) {
        let widgets = self.widgets.borrow();
        let model = self.model.borrow();
        //if flag is dirty
        if let Some(ref state_label) = widgets.state_label {
            state_label.set_text(model.status.as_str());
        }        
    }

    fn start_rendering_timer(&self, self_rc: &Rc<Self>) {
        let self_clone = self_rc.clone();
        gtk::timeout_add(100, move || {
            //Render anything in the render queue
            if let Some(ref rx) = self_clone.channels.borrow().rx {
                while let Ok(state) = rx.try_recv() {
                    println!("Received state in ui updater thread {:?}", state);
                    //let's assume we also got some name/value pairs with enough info to locate our widget and render it
                    
                    //Modifying the model and updating it must be separate scopes
                    {
                        let mut model = self_clone.model.borrow_mut();
                        model.status = format!("{}", state);
                    }                        
                    
                    self_clone.render_dirty(); 
                }
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
    load_button:            Option<gtk::Button>,
    stop_button:            Option<gtk::Button>,
    //load_button_dirty:    bool,
    state_label:            Option<gtk::Label>
}

impl AppWidgets {
    pub fn new() -> AppWidgets {
        AppWidgets {
            app: None,
            win: None,
            file_path_entry: None,
            load_button: None,
            stop_button: None,
            state_label: None
        }
    }
}

// fn current_state() -> State {
//     match UI_STATE.lock() {
//         Ok(inner) => inner.state,
//         Err(e) => {
//             println!("Couldn't acquire UI_STATE lock {:?}", e);
//             State::Error        
//         }
//     }
// }

// fn start_rendering_timer(rx: Rc<Receiver<State>>) {
//     gtk::timeout_add(100, move || {
//         //Render anything in the render queue
//         while let Ok(state) = rx.try_recv() {
//             println!("Received state in ui updater thread {:?}", state);
//             //let's assume we also got some name/value pairs with enough info to locate our widget and render it
//             UI_STATE.lock().unwrap().state = state;  
//             render_dirty(); 
//         }
//         //So this will get invoked on every timeout interval, but the main thread does that anyway by
//         //invoking idle
//         Continue(true)
//     });
// }

// fn render_dirty(/* all the widgets */) {
//     let ui_state = UI_STATE.lock().unwrap();
//     //widget.txt = ui_state.status

//     let factory = GladeObjectFactory::new();
//     let state_label = factory.get::<gtk::Label>("stateLabel");
//     println!("setting text on state_label={:?}", " some new text");
//     state_label.set_text(" some new text");
// }

// fn render(state_label: &gtk::Label, state_arg: State) {
//     state_label.set_text(format!("{}", state_arg).as_str());
// }

// pub fn build(tx: Sender<Command>, rx: Receiver<State>) {

//     let factory = GladeObjectFactory::new();

//     let file_path_entry: Entry = factory.get("filePathEntry1");
//     let state_label = factory.get::<gtk::Label>("stateLabel");
//     //let state_labelx = Arc::new(state_label.clone());

//     //alternative is a thred blockign on recv(), then calling
//     //handle - gtk::idle_add (if possible in that thread)
//     //then idle_remove_by_data(handle) so there is no polling at all

//     let shared_tx = Rc::new(tx);
//     let shared_rx = Rc::new(rx);

//     start_rendering_timer(shared_rx.clone());
    
//     // Create Window   
//     let window : gtk::Window = factory.get("applicationwindow1");
//     window.set_title("castnow desktop-rs");
//     window.connect_delete_event(|_, _| {
//         gtk::main_quit();
//         Inhibit(false)
//     });

    

//     let file_chooser_button: Button = factory.get("popupFileChooserButton");
//     let file_path_entry: Entry = factory.get("filePathEntry1");
//     let (load_button, load_button_tx) = get_obj::<Button>(&factory, "playButton", shared_tx.clone());
//     let (mute_button, mute_button_tx) = get_obj::<Button>(&factory, "muteButton", shared_tx.clone());
//     let (stop_button, stop_button_tx) = get_obj::<Button>(&factory, "stopButton", shared_tx.clone());

//     let open_menu_item = factory.get::<gtk::MenuItem>("openMenuItem");
    
//     let state_label = factory.get::<gtk::Label>("stateLabel");
//     render(&state_label, current_state());

//     let file_path_entry1 = file_path_entry.clone();
//     open_menu_item.connect_activate(move |_| popup_file_chooser(&file_path_entry1));

//     //aboutdialog1

//     let file_path_entry2 = file_path_entry.clone();
//     file_chooser_button.connect_clicked(move |_| popup_file_chooser(&file_path_entry2));
    
//     let state_label2 = state_label.clone();
//     load_button.connect_clicked(move |_| {
//         let path = file_path_entry.get_text();
//         send(&load_button_tx, KeyCommand::Load, path);
//         let new_state = UI_STATE.lock().unwrap().transition_to(KeyCommand::Load);
//         render(&state_label2, new_state);
//     });

//     mute_button.connect_clicked(move |_| send(&mute_button_tx, KeyCommand::Mute, None));
//     let state_label1 = state_label.clone();
//     stop_button.connect_clicked(move |_| {
//         send(&stop_button_tx, KeyCommand::Stop, None);
//         UI_STATE.lock().unwrap().transition_to(KeyCommand::Stop);
//         render(&state_label1, current_state());
//     });
    
//     window.show_all();    
// }

// fn popup_file_chooser(file_path_entry: &Entry) {
//     let dialog = build_file_chooser();  
//     if dialog.run() == gtk::ResponseType::Ok.into() {
//         dialog.get_filename().map(|path| path.to_str().map(|text| file_path_entry.set_text(text)));
//     }
//     dialog.destroy();  
// }

// fn build_file_chooser() -> gtk::FileChooserDialog {
//     let dialog = FileChooserDialog::new (
//                 Some("Choose DireFilectory"),
//                 Some(&Window::new(WindowType::Popup)),
//                 gtk::FileChooserAction::Open,
//     );
//     dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
//     dialog.add_button("Select", gtk::ResponseType::Ok.into());
//     dialog
// }

// fn send(tx: &Sender<Command>, key: KeyCommand, state_arg: Option<String>) {
//     let cmd = Command::new_with_state(key, state_arg.unwrap_or_default());
//     tx.send(cmd).map_err(|e| println!("Failed to send {:?}", e)).ok();
// }

// fn get_obj<T: gtk::IsA<gtk::Object>>(factory: &GladeObjectFactory, name: &'static str, tx: Rc<Sender<Command>>) -> (T, Rc<Sender<Command>>) {
//     return (factory.get(name), tx);
// }
