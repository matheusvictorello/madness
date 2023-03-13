use std::sync::mpsc::{Sender};

use super::ActionCalculation;
use super::Event;

#[derive(Debug)]
pub struct ActionStart {
    pub event_sender: Sender<Event>,
}

impl From<ActionStart> for ActionCalculation {
    fn from(value: ActionStart) -> Self {
        // dbg!("ActionStart into ActionCalculation");

        let ActionStart { event_sender } = value;

        // Run traits with counters
        
        ActionCalculation {
            event_sender,
        }
    }
}
