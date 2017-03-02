use std::sync::mpsc::{Sender, Receiver};
use state::State;
use castnow::{KeyCommand, Command};
use std::rc::Rc;
use std::sync::{Arc};

pub struct UiState {
    pub state: State,
    //more state to be rendered... requires a detailed model of everything
    //to be represented in widgets
    pub status: String
}

impl UiState {
    pub fn new() -> UiState {
        UiState {
            state: State::Initial,
            status: String::default()
        }
    }

    pub fn transition_to(&mut self, cmd: KeyCommand) -> State {
        self.state = State::next(&self.state, &cmd);
        self.state
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
}

pub struct Channel {
    pub tx: Rc<Sender<Command>>,
    pub rx: Rc<Receiver<State>>
}