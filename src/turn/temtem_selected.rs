use std::sync::mpsc::{Sender};

use super::ActionEnd;
use super::DecisionForTemtemSelection;
use super::Event;
use super::Temtem;
use super::TemtemSelection;

#[derive(Debug)]
pub struct TemtemSelected {
    pub event_sender: Sender<Event>,
    pub decision:     DecisionForTemtemSelection,
}

impl TryFrom<TemtemSelected> for ActionEnd {
    type Error = TemtemSelection;

    fn try_from(value: TemtemSelected) -> Result<Self, Self::Error> {
        // dbg!("TemtemSelected try_into ActionEnd");

        let TemtemSelected { event_sender, decision } = value;

        // Validade selection

        // Place temtems on the field

        Ok(ActionEnd {
            event_sender,
        })
    }
}
