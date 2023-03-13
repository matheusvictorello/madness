use std::sync::mpsc::{Sender};

use super::ActionSelection;
use super::Event;
use super::Field;

#[derive(Debug)]
pub struct TurnStart {
    pub event_sender: Sender<Event>,
}

impl From<TurnStart> for ActionSelection {
    fn from(value: TurnStart) -> Self {
        // dbg!("TurnStart into ActionSelection");

        let TurnStart { event_sender } = value;

        // Give stamina to all Temtem

        // Run Traits with 'At the start of the turn'

        // Run Gears with 'At the start of the turn'

        ActionSelection {
            event_sender,
        }
    }
}
