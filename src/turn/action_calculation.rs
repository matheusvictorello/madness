use std::sync::mpsc::{Sender};

use super::ActionExecution;
use super::Event;

#[derive(Debug)]
pub struct ActionCalculation {
    pub event_sender: Sender<Event>,
}

impl From<ActionCalculation> for ActionExecution {
    fn from(value: ActionCalculation) -> Self {
        // dbg!("ActionCalculation into ActionExecution");

        let ActionCalculation { event_sender } = value;

        // Calculate action priority
        
        ActionExecution {
            event_sender,
        }
    }
}
