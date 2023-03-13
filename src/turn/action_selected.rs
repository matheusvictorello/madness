use std::sync::mpsc::{Sender};

use super::ActionSelection;
use super::ActionStart;
use super::DecisionForActionSelection;
use super::Event;

#[derive(Debug)]
pub struct ActionSelected {
    pub event_sender: Sender<Event>,
    pub decision: DecisionForActionSelection,
}

impl TryFrom<ActionSelected> for ActionStart {
    type Error = ActionSelection;

    fn try_from(value: ActionSelected) -> Result<Self, Self::Error> {
        // dbg!("ActionSelected try_into ActionStart");

        let ActionSelected { event_sender, decision } = value;

        // Validade decision

        // Map decision to action list
        // ? Map to new type

        Ok(ActionStart {
            event_sender,
        })
    }
}
