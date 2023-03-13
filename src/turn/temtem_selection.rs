use std::sync::mpsc::{Sender};

use super::Event;
use super::Temtem;
use super::TemtemSelected;

#[derive(Debug)]
pub struct DecisionForTemtemSelection {
    pub team_a: [Option<Temtem>; 2],
    pub team_b: [Option<Temtem>; 2],
}

#[derive(Debug)]
pub struct TemtemSelection {
    pub event_sender: Sender<Event>,
}

impl TemtemSelection {
    pub fn decide(self, decision: DecisionForTemtemSelection) -> TemtemSelected {
        // dbg!("TemtemSelection decided");

        let TemtemSelection { event_sender } = self;

        // ? Nada
        
        TemtemSelected {
            event_sender,
            decision,
        }
    }
}
