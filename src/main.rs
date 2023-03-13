#![feature(adt_const_params)]

use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

mod turn;
use turn::*;

#[derive(Debug)]
enum Team { A, B }

#[derive(Debug)]
enum Winner {
    Team(Team),
    Draw,
}

#[derive(Debug)]
enum Technique {
}

#[derive(Debug)]
enum Targeting {
}

#[derive(Debug)]
enum DecisionRequest {
    ActionSelection(ActionSelection),
    TemtemSelection(TemtemSelection),
}

#[derive(Debug)]
enum Decision {
    ActionSelection(DecisionForActionSelection),
    TemtemSelection(DecisionForTemtemSelection),
}

#[derive(Debug)]
struct TemtemSelectionDecision {}

impl DecisionRequest {
    fn make(self, decision: Decision) -> Result<Phase, Self> {
        match (self, decision) {
            // Valid decision
            (Self::ActionSelection(action_selection), Decision::ActionSelection(decision)) => {
                Ok(action_selection.decide(decision).into())
            }
            (Self::TemtemSelection(temtem_selection), Decision::TemtemSelection(decision)) => {
                Ok(temtem_selection.decide(decision).into())
            }
            // Invalid decision
            (s, _) => {
                Err(s)
            }
        }
    }
}

impl TryFrom<Phase> for Winner {
    type Error = DecisionRequest;

    fn try_from(mut phase: Phase) -> Result<Self, Self::Error> {
        loop {
            phase = match phase {
                Phase::TurnStart(turn_start) => {
                    break Err(DecisionRequest::ActionSelection(turn_start.into()));
                }
                Phase::ActionSelected(action_selected) => {
                    match action_selected.try_into() {
                        Ok(action_start)      => { Phase::ActionStart(action_start) }
                        Err(action_selection) => { break Err(DecisionRequest::ActionSelection(action_selection)); }
                    }
                }
                Phase::ActionStart(action_start) => {
                    Phase::ActionCalculation(action_start.into())
                }
                Phase::ActionCalculation(action_calculation) => {
                    Phase::ActionExecution(action_calculation.into())
                }
                Phase::ActionExecution(action_execution) => {
                    match action_execution.try_into() {
                        Ok(action_end)        => { Phase::ActionEnd(action_end) }
                        Err(temtem_selection) => { break Err(DecisionRequest::TemtemSelection(temtem_selection)); }
                    }
                }
                Phase::TemtemSelected(temtem_selection) => {
                    match temtem_selection.try_into() {
                        Ok(action_end)        => { Phase::ActionEnd(action_end) }
                        Err(temtem_selection) => { break Err(DecisionRequest::TemtemSelection(temtem_selection)); }
                    }
                }
                Phase::ActionEnd(action_end) => {
                    Phase::TurnEnd(action_end.into())
                }
                Phase::TurnEnd(turn_end) => {
                    match turn_end.try_into() {
                        Ok(winner)      => { break Ok(winner); }
                        Err(turn_start) => { Phase::TurnStart(turn_start) }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Temtem {
    Cerneaf,
    Nagaise,
    Barnshe,
    Oceara,
}

#[derive(Debug)]
struct Field<'d, 'b> {
    team_a:         TeamInBattle<'d, 'b>,
    team_b:         TeamInBattle<'d, 'b>,
    turn:           Phase,
    event_receiver: Receiver<Event>,
}

#[derive(Debug)]
struct TeamInBattle<'d, 'b> {
    temtems: [TemtemInBattle<'d, 'b>; 2],
    left:    Option<Temtem>,
    right:   Option<Temtem>,
}

#[derive(Debug)]
struct TemtemInBattle<'d, 'b: 'd> {
    build: &'b TemtemBuild<'d>,
    hp:    u32,
}

#[derive(Debug)]
struct TemtemBuild<'d> {
    data: &'d TemtemData,
}

#[derive(Debug)]
struct TemtemData {
    temtem: Temtem,
}

#[derive(Debug)]
struct CompetitiveTeam<'d, 'b> {
    temtems: [&'b TemtemBuild<'d>; 2],
}

impl <'d, 'b: 'd> TemtemInBattle<'d, 'b> {
    fn temtem(&self) -> Temtem {
        self.build.data.temtem.clone()
    }
}

pub enum Event {

}

impl <'d, 'b> Field<'d, 'b> {
    fn new(team_a: CompetitiveTeam<'d, 'b>, team_b: CompetitiveTeam<'d, 'b>) -> Self {

        let (event_sender, event_receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();

        let team_a = Self::build_team(team_a);
        let team_b = Self::build_team(team_b);
        let turn = Phase::TemtemSelected(TemtemSelected {
            event_sender,
            decision: DecisionForTemtemSelection {
                team_a: [Some(team_a.temtems[0].temtem()), Some(team_a.temtems[1].temtem())],
                team_b: [Some(team_b.temtems[0].temtem()), Some(team_b.temtems[1].temtem())],
            },
        });

        Self {
            team_a,
            team_b,
            turn,
            event_receiver,
        }
    }

    fn build_team(team: CompetitiveTeam<'d, 'b>) -> TeamInBattle<'d, 'b> {
        let [t0, t1] = team.temtems;

        TeamInBattle {
            temtems: [
                Self::build_temtem(t0),
                Self::build_temtem(t1),
            ],
            left: None,
            right: None,
        }
    }

    fn build_temtem(temtem_build: &'b TemtemBuild<'d>) -> TemtemInBattle<'d, 'b> {
        TemtemInBattle {
            build: temtem_build,
            hp: 100,
        }
    }
}

fn main() {
    let temtem_data = {
        let mut hm: HashMap<Temtem, TemtemData> = HashMap::new();

        hm.insert(Temtem::Cerneaf, TemtemData {
            temtem: Temtem::Cerneaf,
        });

        hm.insert(Temtem::Nagaise, TemtemData {
            temtem: Temtem::Nagaise,
        });

        hm.insert(Temtem::Barnshe, TemtemData {
            temtem: Temtem::Barnshe,
        });

        hm.insert(Temtem::Oceara, TemtemData {
            temtem: Temtem::Oceara,
        });

        hm
    };

    let team_a = CompetitiveTeam {
        temtems: [
            &TemtemBuild {
                data: temtem_data.get(&Temtem::Cerneaf).unwrap(),
            },
            &TemtemBuild {
                data: temtem_data.get(&Temtem::Cerneaf).unwrap(),
            },
        ],
    };

    let team_b = CompetitiveTeam {
        temtems: [
            &TemtemBuild {
                data: temtem_data.get(&Temtem::Nagaise).unwrap(),
            },
            &TemtemBuild {
                data: temtem_data.get(&Temtem::Nagaise).unwrap(),
            },
        ],
    };

    let mut field = Field::new(team_a, team_b);

    println!("{:#?}", field);

    // let mut phase = Phase::TemtemSelected(TemtemSelected {});

    // let winner: Winner = loop {
    //     phase = match phase.try_into() {
    //         Ok(winner) => {
    //             break winner;
    //         }
    //         Err(decision) => {
    //             decision.make(Decision::TemtemSelection(TemtemSelectionDecision {}))
    //         }
    //     }
    // };
}