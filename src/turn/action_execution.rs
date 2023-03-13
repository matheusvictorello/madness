use std::sync::mpsc::{Sender};

use super::ActionEnd;
use super::Event;
use super::TemtemSelection;

#[derive(Debug)]
pub struct ActionExecution {
    pub event_sender: Sender<Event>,
}

impl TryFrom<ActionExecution> for ActionEnd {
    type Error = TemtemSelection;
    
    fn try_from(value: ActionExecution) -> Result<Self, Self::Error> {
        // dbg!("ActionExecution try_into ActionEnd");

        let ActionExecution { event_sender } = value;

        // Apply actions in order

        // TODO: can fail and go to TemtemSelection
        
        Ok(ActionEnd {
            event_sender,
        })
    }
}
