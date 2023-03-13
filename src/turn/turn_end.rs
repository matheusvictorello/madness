use std::sync::mpsc::{Sender};

use super::Event;
use super::TurnStart;
use super::Winner;

#[derive(Debug)]
pub struct TurnEnd {
    pub event_sender: Sender<Event>,
}

impl TryFrom<TurnEnd> for Winner {
    type Error = TurnStart;
    
    fn try_from(value: TurnEnd) -> Result<Self, Self::Error> {
        // dbg!("TurnEnd try_into Winner");

        let TurnEnd { event_sender } = value;
        
        // Check victory

        // Increment turn counter

        // TODO: can fail and go to TurnStart
        
        Ok(Winner::Draw)
    }
}