use std::sync::mpsc::{Sender};

use super::ActionSelected;
use super::Event;
use super::Targeting;
use super::Technique;
use super::Temtem;

#[derive(Debug)]
pub enum Action {
    Swap(Temtem),
    Tech(Technique, Targeting),
    Rest,
}

#[derive(Debug)]
pub struct DecisionForActionSelection {
    pub team_a: [Action; 2],
    pub team_b: [Action; 2],
}

#[derive(Debug)]
pub struct ActionSelection {
    pub event_sender: Sender<Event>,
}

impl ActionSelection {
    pub fn decide(self, decision: DecisionForActionSelection) -> ActionSelected {
        // dbg!("ActionSelection decided");

        let ActionSelection { event_sender } = self;

        // ? Nada

        ActionSelected {
            event_sender,
            decision,
        }
    }
}
