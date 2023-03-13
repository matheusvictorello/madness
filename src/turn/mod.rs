use super::Event;
use super::Field;
use super::Targeting;
use super::Technique;
use super::Temtem;
use super::Winner;

mod action_calculation;
pub use action_calculation::ActionCalculation;

mod action_end;
pub use action_end::ActionEnd;

mod action_execution;
pub use action_execution::ActionExecution;

mod action_selected;
pub use action_selected::ActionSelected;

mod action_selection;
pub use action_selection::DecisionForActionSelection;
pub use action_selection::ActionSelection;

mod action_start;
pub use action_start::ActionStart;

mod temtem_selected;
pub use temtem_selected::TemtemSelected;

mod temtem_selection;
pub use temtem_selection::DecisionForTemtemSelection;
pub use temtem_selection::TemtemSelection;

mod turn_end;
pub use turn_end::TurnEnd;

mod turn_start;
pub use turn_start::TurnStart;

#[derive(Debug)]
pub enum Phase {
    TurnStart(TurnStart),
    // Start of turn traits

    ActionSelected(ActionSelected),
    // Receive and validate user action

    ActionStart(ActionStart),
    // Traits with counters

    ActionCalculation(ActionCalculation),
    // Enqueue actions

    ActionExecution(ActionExecution),
    // Exec actions in order

    TemtemSelected(TemtemSelected),
    // Receive and validate user action
    // Temtems enter

    ActionEnd(ActionEnd),
    // Trait with counter update
    // Conditions run

    TurnEnd(TurnEnd),
    // Check victory
    // Increment counter
}

impl From<TurnStart> for Phase {
    fn from(value: TurnStart) -> Self {
        Self::TurnStart(value)
    }
}

impl From<ActionSelected> for Phase {
    fn from(value: ActionSelected) -> Self {
        Self::ActionSelected(value)
    }
}

impl From<ActionStart> for Phase {
    fn from(value: ActionStart) -> Self {
        Self::ActionStart(value)
    }
}

impl From<ActionCalculation> for Phase {
    fn from(value: ActionCalculation) -> Self {
        Self::ActionCalculation(value)
    }
}

impl From<ActionExecution> for Phase {
    fn from(value: ActionExecution) -> Self {
        Self::ActionExecution(value)
    }
}

impl From<TemtemSelected> for Phase {
    fn from(value: TemtemSelected) -> Self {
        Self::TemtemSelected(value)
    }
}

impl From<ActionEnd> for Phase {
    fn from(value: ActionEnd) -> Self {
        Self::ActionEnd(value)
    }
}

impl From<TurnEnd> for Phase {
    fn from(value: TurnEnd) -> Self {
        Self::TurnEnd(value)
    }
}