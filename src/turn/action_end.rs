use std::sync::mpsc::{Sender};

use super::Event;
use super::TurnEnd;

#[derive(Debug)]
pub struct ActionEnd {
    pub event_sender: Sender<Event>,
}

impl From<ActionEnd> for TurnEnd {
    fn from(value: ActionEnd) -> Self {
        // dbg!("ActionEnd into TurnEnd");

        let ActionEnd { event_sender } = value;

        // Update counter from traits with counter

        // Run conditions
        
        TurnEnd {
            event_sender,
        }
    }
}
