#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::min;
use std::marker::PhantomData;
use std::collections::HashMap;

// trait State {
//     type Event;

//     fn modifier(&self) -> Box<dyn Fn(Self::Event) -> Self::Event>;
// }

// trait Event {
//     type State;

//     fn apply(self);
// }

// trait Actor {
//     type State;
//     type Event;

//     fn what_now(&self, state: Rc<RefCell<Self::State>>) -> Option<Vec<Self::Event>>;
// }

// struct Controller<S: State, A: Actor> {
//     state: Rc<RefCell<S>>,
//     actor: A,
// }

// impl <S, E, A> Controller<S, A> where S: State<Event = E>, E: Event<State = S>, A: Actor<State = S, Event = E>,  {
//     fn new(state: S, actor: A) -> Self {
//         Self {
//             state: Rc::new(RefCell::new(state)),
//             actor,
//         }
//     }

//     fn run(mut self) -> Rc<RefCell<S>> {
//         loop {
//             match self.actor.what_now(self.state.clone()) {
//                 Some(events) => {
//                     for event in events.into_iter() {
//                         self.state
//                             .borrow()
//                             .modifier()(event)
//                             .apply();
//                     }
//                 }
//                 None => {
//                     break self.state;
//                 }
//             }
//         }
//     }
// }

// //

// #[derive(Debug)]
// struct MyActor {

// }

// #[derive(Debug)]
// enum Team {
//     A,
//     B,
// }

// #[derive(Debug)]
// struct CompetitiveTemtem {

// }

// #[derive(Debug)]
// struct CompetitiveTemtemInBattle {
//     hp: u32,
// }

// #[derive(Debug)]
// struct CompetitiveTeam {
//     temtems: [CompetitiveTemtem; 1],
// }

// #[derive(Debug)]
// struct CompetitiveTeamInBattle {
//     temtems: [CompetitiveTemtemInBattle; 1],
//     left:    Option<usize>,
//     right:   Option<usize>,
// }

// #[derive(Debug)]
// enum Battle {
//     Pre {
//         team_a: CompetitiveTeam,
//         team_b: CompetitiveTeam,
//     },
//     Fighting {
//         team_a:   CompetitiveTeamInBattle,
//         team_b:   CompetitiveTeamInBattle,
//         priority: Team,
//     },
//     Finished {
//         winner: Team,
//     }
// }

// #[derive(Debug)]
// enum MyEvent {
//     BattleStart { state: Rc<RefCell<Battle>> },
//     TurnAction,
// }

// impl Actor for MyActor {
//     type State = Battle;
//     type Event = MyEvent;

//     fn what_now(&self, state: Rc<RefCell<Self::State>>) -> Option<Vec<Self::Event>> {
//         match *state {
//             Self::State::Pre { team_a, team_b } => {
//                 Some(vec![
//                     Self::Event::BattleStart { state: state.clone() }
//                 ])
//             }
//             Self::State::Fighting { team_a, team_b, priority } => {
//                 Some(vec![
//                     Self::Event::TurnAction
//                 ])
//             }
//             _ => {
//                 None
//             }
//         }
//     }
// }

// impl State for Battle {
//     type Event = MyEvent;

//     fn modifier(&self) -> Box<dyn Fn(Self::Event) -> Self::Event> {
//         Box::new(|event: Self::Event| {
//             event
//         })
//     }
// }

// impl Event for MyEvent {
//     type State = Battle;

//     fn apply(self) {
//         match self {
//             Self::BattleStart { state } => {
//                 let mut state = state.borrow_mut();
//                 *state = Self::State::Fighting {
//                     team_a: CompetitiveTeamInBattle {
//                         temtems: [
//                             CompetitiveTemtemInBattle {
//                                 hp: 100,
//                             },
//                         ],
//                         left:  None,
//                         right: None,
//                     },
//                     team_b: CompetitiveTeamInBattle {
//                         temtems: [
//                             CompetitiveTemtemInBattle {
//                                 hp: 100,
//                             },
//                         ],
//                         left:  None,
//                         right: None,
//                     },
//                     priority: Team::A,
//                 }
//             }
//             Self::TurnAction => {

//             }
//         }
//     }
// }

// struct GameLogic {

// }

// impl GameLogic {
//     fn what_now() -> {

//     }
// }

// struct Battle {
//     pool: [Box<dyn Event<State = Self>>; 2],
//     curr: usize,
// }

// trait Event {
//     type State;

//     fn unwrap(self) -> fn(Self::State) -> Self::State;
// }

// struct GenericTechnique {}

// impl Event for GenericTechnique {
//     type State = Battle;

//     fn unwrap(self) -> fn(Self::State) -> Self::State {
//         |state| {
//             state
//         }
//     }
// }

// struct GenericTrait {}

// impl Event for GenericTrait {
//     type State = Battle;

//     fn unwrap(self) -> fn(Self::State) -> Self::State {
//         |state| {
//             state
//         }
//     }
// }



// enum TeamInBattleSE {
//     Left(Option<usize>),
//     Right(Option<usize>),
// }



// enum BattleSE {
//     TeamA(TeamInBattleSE),
//     TeamB(TeamInBattleSE),
// }

// struct Battle {
//     team_a: TeamInBattle,
//     team_b: TeamInBattle,
// }

// enum SingleOtherTargeting {
//     Ally,
//     LeftEnemy,
//     RightEnemy,
// }

// enum Targeting {
//     SingleOther(SingleOtherTargeting),
// }

// enum Technique {
// }

// enum TemtemAction {
//     Swap(usize),
//     Tech(Technique, Targeting),
//     Rest,
// }

// struct TeamAction {
//     left:  TemtemAction,
//     right: TemtemAction,
// }

// struct TurnAction {
//     team_a: TeamAction,
//     team_b: TeamAction,
// }

// trait SideEffect {
//     fn unwrap(self: Box<Self>);
// }

// enum EventResult {
//     Event(Box<dyn Event>),
//     SideEffect(Box<dyn SideEffect>),
// }

// trait Event {
//     fn unwrap(self: Box<Self>) -> Vec<EventResult>;
// }

// //

// #[derive(Debug)]
// struct Stats {
//     hp: u32,
// }

// #[derive(Debug)]
// struct TemtemInBattle {
//     stats: Stats,
// }

// #[derive(Debug)]
// struct TeamInBattle {
//     temtems: [Rc<RefCell<TemtemInBattle>>; 5],
//     left:    Option<Rc<RefCell<TemtemInBattle>>>,
//     right:   Option<Rc<RefCell<TemtemInBattle>>>,
// }

// #[derive(Debug, Clone)]
// enum Position {
//     Left,
//     Right,
// }

// #[derive(Debug, Clone)]
// struct Swap {
//     team:     Rc<RefCell<TeamInBattle>>,
//     temtem:   Rc<RefCell<TemtemInBattle>>,
//     position: Position,
// }

// impl SideEffect for Swap {
//     fn unwrap(self: Box<Self>) {
//         let Swap { team, temtem, position } = *self;

//         match position {
//             Position::Left  => {
//                 team.borrow_mut().left  = Some(temtem);
//             }
//             Position::Right => {
//                 team.borrow_mut().right = Some(temtem);
//             }
//         }
//     }
// }

// #[derive(Debug, Clone)]
// struct Damage {
//     value:  u32,
//     target: Rc<RefCell<TemtemInBattle>>,
// }

// impl SideEffect for Damage {
//     fn unwrap(self: Box<Self>) {
//         let Damage { value, target } = *self;

//         let mut hp = &mut target.borrow_mut().stats.hp;

//         *hp += min(value, *hp);
//     }
// }


// #[derive(PartialEq, Eq)]
// enum Technique {
//     A,
//     B,
//     C,
// }

// enum SingleOtherTarget {
//     Ally(Rc<RefCell<TemtemInBattle>>),
//     LeftEnemy(Rc<RefCell<TemtemInBattle>>),
//     RightEnemy(Rc<RefCell<TemtemInBattle>>),
// }

// enum OtherTeamOrAlly {
//     Ally(Rc<RefCell<TemtemInBattle>>),
//     OtherTeam {
//         left:  Rc<RefCell<TemtemInBattle>>,
//         right: Rc<RefCell<TemtemInBattle>>,
//     }
// }

// struct TechniqueImpl<const Tc: Technique, Tr> {
//     user:   Rc<RefCell<TemtemInBattle>>,
//     target: Tr,
// }

// impl TechniqueImpl<{Technique::A}, SingleOtherTarget> {
//     fn new(user: Rc<RefCell<TemtemInBattle>>, target: SingleOtherTarget) -> Self {
//         Self {
//             user,
//             target,
//         }
//     }
// }

// impl SideEffect for TechniqueImpl<{Technique::A}, SingleOtherTarget> {
//     fn unwrap(self: Box<Self>) {
//         let TechniqueImpl { user, target } = *self;

//         let target = match target {
//             SingleOtherTarget::Ally(target)       => { target }
//             SingleOtherTarget::LeftEnemy(target)  => { target }
//             SingleOtherTarget::RightEnemy(target) => { target }
//         };


//     }
// }



//
// mod trait {
//     struct Adaptive; impl Adaptive {
//         fn on_event(&'battle mut self, event: Event) -> Option<Vec<?>> {
//             // Once per battle
//             if self.used {
//                 return None;
//             }
//             self.used = true;

//             match event {
//                 // After being attacked
//                 Done(Technique(technique, _user, targeting)) if targeting.contains(self.owner) {
//                     Some(vec![
//                         // Changes the Temtem's second type to the type of the received technique
//                         SecondType(self.owner, technique.technique_type),
//                         // Gets SPATK++
//                         StatsChange(self.owner, Stats::SPATK, 2),
//                     ])
//                 }
//                 _ => {
//                     None
//                 }
//             }
//         }
//     }

//     struct Aerobic; impl Aerobic {
//         fn on_event(&'battle mut self, event: Event) -> Option<Vec<?>> {
//             match event {
//                 // After attacking with a Wind technique
//                 Done(Technique(technique, user, targeting)) if user == self.owner {
//                     Some(vec![
//                         // Gets SPDEF-- 
//                         StatsChange(self.owner, Stats::SPDEF, -2),
//                         // And SPD+
//                         StatsChange(self.owner, Stats::SPD, 1),
//                     ])
//                 }
//                 _ => {
//                     None
//                 }
//             }
//         }
//     }
// }

// struct SomeTrait {
//     owner: &TemtemInBattle,
//     used:  bool,
// }

// impl SomeTrait {
//     fn on_event(&mut self, event: Event) {
        
//     }
// }

// actor wants to apply side effect

// side effect unwraps

// side effect unwraps to more side effects to be applied 

// new side effects need to be accepted by the current listeners

// new side effects

// struct SideEffect<S> {
//     fs: Vec<fn(S) ->  S>,
// }

// impl <S> SideEffect<S> {
//     fn apply(self, mut s: S) -> S {
//         let Self { fs } = self;

//         for f in fs.into_iter() {
//             s = f(s)
//         }

//         s
//     }
// }

// struct Stats {
//     hp: u32,
// }

// struct TemtemData {
//     temtem: Temtem,
// }

// struct TemtemBuild<'d> {
//     data: &'d TemtemData,
// }

// struct TemtemInBattle<'d, 'b> {
//     build: &'b TemtemBuild<'d>,
//     stats: Stats,
// }

// struct TeamInBattle<'d, 'b> {
//     temtems: [TemtemInBattle<'d, 'b>; 5],
//     left:    Option<usize>,
//     right:   Option<usize>,
// }

// struct Battle<'d, 'b> {
//     team_a: TeamInBattle<'d, 'b>,
//     team_b: TeamInBattle<'d, 'b>,
// }

// // Intent layer
// struct TurnAction {
//     team_a: TeamAction,
//     team_b: TeamAction,
// }

// struct TeamAction {
//     left:  TemtemAction,
//     right: TemtemAction,
// }

// enum TemtemAction {
//     Swap(Temtem),
//     Tech(Technique, Targeting),
//     Rest,
// }

// #[derive(PartialEq, Eq)]
// enum Temtem {
//     Cerneaf,
// }

// enum Technique {
//     Bush,
// }

// enum Targeting {
//     SingleOther(SingleOtherTarget),
//     SingleTeam(SingleTeamTarget),
// }

// enum SingleOtherTarget {
//     Ally,
//     EnemyLeft,
//     EnemyRight,
// }

// enum SingleTeamTarget {
//     Own,
//     Enemy,
// }

// Reference layer

// enum Action {
//     // Swap {
//     //     position: Position,
//     //     index:    usize,
//     // },
//     Tech {
//         user:      &TemtemInBattle,
//         target:    &TemtemInBattle,
//         technique: &TechniqueData,
//     },
//     // Rest {
//     //     position: Position,
//     // },
// }

// struct TechniqueData {

// }

// fn f0(action: TurnAction, battle: &Battle) -> [(); 4] {
//     let TurnAction { team_a: team_a_action, team_b: team_b_action } = action;
//     let Battle { team_a, team_b } = battle;

//     f1(team_a_action, team_a);
//     f1(team_b_action, team_b);

//     [(); 4]
// }

// fn f1(action: TeamAction, team: &TeamInBattle) -> [(); 2] {
//     let TeamAction { left: left_action, right: right_action } = action;
//     let TeamInBattle { temtems, left, right } = team;

//     // [
//     //     match left {
//     //         Some(usize) => {
//     //             f2(left_action)
//     //         }
//     //         None => {
//     //             None
//     //         }
//     //     },
//     //     match right {
//     //         Some(usize) => {
//     //             f2(right_action)
//     //         }
//     //         None => {
//     //             None
//     //         }
//     //     },
//     // ];

//     [(); 2]
// }

// fn f2(action: TemtemAction, temtems: &[&TemtemInBattle; 5]) -> () {
//     match action {
//         TemtemAction::Swap(temtem) => {
//             let (index, _) = temtems.iter()
//                 .enumerate()
//                 .filter(|(idx, tt)| temtem == tt.build.data.temtem)
//                 .next();

//             match index {
//                 Some(index) => {
//                     Some(?::Swap(index))
//                 }
//                 None => {
//                     None
//                 }
//             }
//         }
//         TemtemAction::Tech(technique, targeting) => {
//             // todo
//         }
//         TemtemAction::Rest => {
//             // todo
//         }
//     }
// }

// trait Decidable {
//     type Decision;

//     fn decide(&mut self, decision: Self::Decision);
// }

// enum TurnPhase {
//     A,
//     B,
//     C,
// }

// struct Turn<const P: TurnPhase> {

// }

// impl Decidable for Turn<{TurnPhase::A}> {
//     type Decision = u32;

//     fn decide(&mut self, decision: Self::Decision) {

//     }
// }

// impl Decidable for Turn<{TurnPhase::B}> {
//     type Decision = bool;

//     fn decide(&mut self, decision: Self::Decision) {

//     }
// }

// #[derive(Debug)]
// struct Battle {
//     turn: Turn,
// }

// #[derive(Debug)]
// struct CompetitiveTeam {}

// impl Battle {
//     fn new(team_a: CompetitiveTeam, team_b: CompetitiveTeam) -> Self {
//         Self {
//             turn: Turn<{TurnPhase::A}> {

//             }
//         }
//     }

//     fn decide(&mut self, decision: Decision) {
//         self.decision = Some(decision);
//     }

//     fn step(&mut self) -> Option<Decision> {
//         let Self { count, decision } = self;

//         match (*count % 2 == 0, decision) {
//             (true, Some(decision)) => {
//                 match (*count, decision) {
//                     (4, Decision::A) => {
//                         self.count -= 1;
//                         None
//                     }
//                     (4, _) => {
//                         Some(Decision::A)
//                     }
//                     (2, Decision::B) => {
//                         self.count -= 1;
//                         None
//                     }
//                     (2, _) => {
//                         Some(Decision::B)
//                     }
//                     (0, Decision::C) => {
//                         self.count -= 1;
//                         None
//                     }
//                     (0, _) => {
//                         Some(Decision::C)
//                     }
//                     (_, _) => {
//                         None
//                     }
//                 }
//             }
//             (true, None) => {
//                 Some(Decision::A)
//             }
//             (false, _) => {
//                 self.count -= 1;
//                 None
//             }
//         }
//     }
// }

// TemtemData = TechniqueData > TemtemBuild > Battle => TemtemInBattle = TechniqueInBattle = Field > Turn > Action = Event = SideEffect

// enum Thing {
//     A,
//     B,
//     C,
// }

// const fn f(t0: Thing, t1: Thing) -> u32 {
//     let a = match t0 {
//         Thing::A => 0,
//         Thing::B => 1,
//         Thing::C => 2,
//     };

//     let b = match t1 {
//         Thing::A => 1,
//         Thing::B => 2,
//         Thing::C => 3,
//     };

//     a + b
// }

#[derive(Debug)]
struct Field<T> {
    team_a_left:  Option<T>,
    team_a_right: Option<T>,
    team_b_left:  Option<T>,
    team_b_right: Option<T>,
}

type ActionSelection = Field<Action>;
type TemtemSelection = Field<Temtem>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Temtem {
    Cerneaf,
}

#[derive(Debug)]
enum Technique {

}

#[derive(Debug)]
enum Targeting {

}

#[derive(Debug)]
enum Action {
    Swap(Temtem),
    Tech(Technique, Targeting),
    Rest,
}

#[derive(Debug)]
struct CompetitiveTeamInBattle<'d> {
    temtems: [TemtemInBattle<'d>; 2],
}

impl <'d> CompetitiveTeamInBattle<'d> {
    fn has(&'d self, temtem: Temtem) -> bool {
        true
    }
}

#[derive(Debug)]
struct TemtemInBattle<'d> {
    build: TemtemBuild<'d>,
    hp:    u32,
}

#[derive(Debug)]
enum TurnPhase {
    FirstPhase,
    // > First tems enter

    StartOfTurn,
    // > Run traits with "at start of turn"

    ActionSelection,
    // > Receive user action

    StartOfAction,
    // > Trait with count pop up, based on tems in the field

    TurnOrderCalculation,
    // > Enqueue actions

    TurnOrderExecution,
    // > Exec actions in order

    PostMovePhase,
    // > New tems enter
    // > Trait counter update
    // > Conditions run

    EndOfTurn,
    // > Check victory
    // > Increment turn counter

    LastPhase(Winner),
    // Has winner
}

#[derive(Debug)]
struct Battle<'d> {
    team_a:       CompetitiveTeamInBattle<'d>,
    team_b:       CompetitiveTeamInBattle<'d>,
    field:        Field<Temtem>,
    decision:     Option<BattleDecision>,
    turn_phase:   TurnPhase,
    current_turn: u32,
}

impl <'d> Battle<'d> {
    fn new(team_a: [TemtemBuild<'d>; 2], team_b: [TemtemBuild<'d>; 2]) -> Self {
        let team_a_in_battle = Self::build_team_in_battle(team_a);
        let team_b_in_battle = Self::build_team_in_battle(team_b);

        let team_a = CompetitiveTeamInBattle {
            temtems: team_a_in_battle,
        };

        let team_b = CompetitiveTeamInBattle {
            temtems: team_b_in_battle,
        };

        let field = Field {
            team_a_left:  None,
            team_a_right: None,
            team_b_left:  None,
            team_b_right: None,
        };

        Self {
            team_a:       team_a,
            team_b:       team_b,
            field:        field,
            decision:     None,
            turn_phase:   TurnPhase::FirstPhase,
            current_turn: 0,
        }
    }

    fn build_team_in_battle(team: [TemtemBuild; 2]) -> [TemtemInBattle; 2] {
        let [t0, t1] = team;

        [
            TemtemInBattle {
                build: t0,
                hp:    100,
            },
            TemtemInBattle {
                build: t1,
                hp:    100,
            },
        ]
    }

    fn step(&mut self) -> Option<BattleDecisionRequest> {
        let Self {
            team_a,
            team_b,
            field,
            decision,
            turn_phase,
            current_turn,
        } = self;

        match turn_phase {
            TurnPhase::FirstPhase => {
                println!("FirstPhase");

                // > First tems enter
                field.team_a_left  = Some(team_a.temtems[0].build.data.temtem);
                field.team_a_right = Some(team_a.temtems[1].build.data.temtem);
                field.team_b_left  = Some(team_b.temtems[0].build.data.temtem);
                field.team_b_right = Some(team_b.temtems[1].build.data.temtem);

                // Next phase
                *turn_phase = TurnPhase::EndOfTurn;
                
                // No decision needed
                None
            }
            TurnPhase::StartOfTurn => {
                println!("StartOfTurn");

                // > Run traits with "at start of turn"
                // TODO

                // Next phase
                *turn_phase = TurnPhase::ActionSelection;

                // No decision needed
                None
            }
            TurnPhase::ActionSelection => {
                println!("ActionSelection");

                // > Receive user action
                match decision.take() {
                    Some(decision) => {
                        // Next phase
                        *turn_phase = TurnPhase::StartOfAction;
                        // No decision needed
                        None
                    }
                    None => {
                        // Decision needed
                        Some(BattleDecisionRequest::ActionSelection)
                    }
                }
            }
            TurnPhase::StartOfAction => {
                println!("StartOfAction");

                // > Trait with count pop up, based on tems in the field
                // TODO

                // Next phase
                *turn_phase = TurnPhase::TurnOrderCalculation;

                // No decision needed
                None
            }
            TurnPhase::TurnOrderCalculation => {
                println!("TurnOrderCalculation");

                // > Enqueue actions
                // TODO

                // Next phase
                *turn_phase = TurnPhase::TurnOrderExecution;

                // No decision needed
                None
            }
            TurnPhase::TurnOrderExecution => {
                println!("TurnOrderExecution");

                // > Exec actions in order
                // TODO

                // Next phase
                *turn_phase = TurnPhase::PostMovePhase;

                // No decision needed
                None
            }
            TurnPhase::PostMovePhase => {
                println!("PostMovePhase");

                let need_swap_ins = {
                    false
                    || field.team_a_left  == None
                    || field.team_a_right == None
                    || field.team_b_left  == None
                    || field.team_b_right == None
                };

                if need_swap_ins {
                    match decision.take() {
                        Some(BattleDecision::TemtemSelection(decision)) => {
                            // > New tems enter
                            let TemtemSelection {
                                team_a_left,
                                team_a_right,
                                team_b_left,
                                team_b_right,
                            } = decision;

                            // TODO impl abort

                            match (field.team_a_left, team_a_left) {
                                (None, Some(temtem)) => {
                                    if team_a.has(temtem) {
                                        field.team_a_left = Some(temtem);
                                    }
                                }
                                _ => {}
                            }

                            match (field.team_a_right, team_a_right) {
                                (None, Some(temtem)) => {
                                    if team_a.has(temtem) {
                                        field.team_a_right = Some(temtem);
                                    }
                                }
                                _ => {}
                            }

                            match (field.team_b_left, team_b_left) {
                                (None, Some(temtem)) => {
                                    if team_a.has(temtem) {
                                        field.team_b_left = Some(temtem);
                                    }
                                }
                                _ => {}
                            }

                            match (field.team_b_right, team_b_right) {
                                (None, Some(temtem)) => {
                                    if team_a.has(temtem) {
                                        field.team_b_right = Some(temtem);
                                    }
                                }
                                _ => {}
                            }

                            // > Trait counter update
                            // TODO

                            // > Conditions run
                            // TODO

                            // Next phase
                            *turn_phase = TurnPhase::EndOfTurn;

                            // No decision needed
                            None
                        }
                        _ => {
                            // Decision needed
                            Some(BattleDecisionRequest::TemtemSelection)
                        }
                    }
                } else {
                    // Next phase
                    *turn_phase = TurnPhase::EndOfTurn;

                    // No decision needed
                    None
                }
            }
            TurnPhase::EndOfTurn => {
                println!("EndOfTurn");

                // > Check victory
                if *current_turn >= 30 {
                    // Next phase
                    // TODO undraw conditions
                    *turn_phase = TurnPhase::LastPhase(Winner::Draw); 
                } else {
                    // > Increment turn counter
                    *current_turn += 1;

                    println!("current_turn {:?}", current_turn);
                    println!("turn_phase {:?}", turn_phase);

                    // Next phase
                    *turn_phase = TurnPhase::StartOfTurn;
                }

                // No decision needed
                None
            }
            TurnPhase::LastPhase(_winner) => {
                println!("LastPhase");

                // Battle finished
                Some(BattleDecisionRequest::EndBattle)
            }
        }
    }

    fn decide(&mut self, decision: BattleDecision) {
        self.decision = Some(decision);
    }
}

#[derive(Debug)]
enum BattleDecision {
    ActionSelection(ActionSelection),
    TemtemSelection(TemtemSelection),
}

#[derive(Debug)]
enum BattleDecisionRequest {
    ActionSelection,
    TemtemSelection,
    EndBattle,
}

#[derive(Debug)]
enum Team {
    A,
    B,
}

#[derive(Debug)]
struct TemtemData {
    temtem: Temtem,
}

#[derive(Debug)]
struct TemtemBuild<'d> {
    data: &'d TemtemData,
}

#[derive(Debug)]
enum Winner {
    Team(Team),
    Draw,
}

fn main() {

    let temtem_data = {
        let mut hs = HashMap::new();

        hs.insert(Temtem::Cerneaf, TemtemData {
            temtem: Temtem::Cerneaf,
        });

        hs
    };

    let team_a = [
        TemtemBuild {
            data: temtem_data.get(&Temtem::Cerneaf).expect("Missing Cerneaf data")
        },
        TemtemBuild {
            data: temtem_data.get(&Temtem::Cerneaf).expect("Missing Cerneaf data")
        },
    ];

    let team_b = [
        TemtemBuild {
            data: temtem_data.get(&Temtem::Cerneaf).expect("Missing Cerneaf data")
        },
        TemtemBuild {
            data: temtem_data.get(&Temtem::Cerneaf).expect("Missing Cerneaf data")
        },
    ];

    let mut battle = Battle::new(team_a, team_b);

    loop {
        let decision = battle.step();

        match decision {
            Some(BattleDecisionRequest::ActionSelection) => {
                let decision = BattleDecision::ActionSelection(ActionSelection {
                    team_a_left:  Some(Action::Rest),
                    team_a_right: None,
                    team_b_left:  Some(Action::Rest),
                    team_b_right: None,
                });

                battle.decide(decision);
            }
            Some(BattleDecisionRequest::TemtemSelection) => {

            }
            Some(BattleDecisionRequest::EndBattle) => {
                break;
            }
            None => {}
        }
    }

    println!("{:#?}", battle);

    // let a1 = TemtemSelection {
    //     team_a_left:  None,
    //     team_a_right: None,
    //     team_b_left:  None,
    //     team_b_right: None,
    // };

    // let result: u32 = f(Thing::A, Thing::B);

    // println!("{:?}", result);

    // let temtem_data: HashMap<Temtem, TemtemData> = {
    //     let hm = HashMap::new();

    //     hm.insert(Temtem::Cerneaf, TemtemData {
    //         temtem: Temtem::Cerneaf,
    //     });

    //     hm
    // };

    // enum Phase {
    //     A,
    //     B,
    //     C,
    // };

    // struct Battle {

    // }

    // enum Thing {
    //     MoveSelection(MoveSelection),
    //     TemtemSelection(TemtemSelection),
    // }

    // {
    //     let mut b = Battle::new(CompetitiveTeam {}, CompetitiveTeam {});
    //     {
    //         PostMove
    //     }

    //     b.decide(Thing);
    // }

    // let dummy_temtem_data = TemtemData {
    //     temtem: Temtem::Cerneaf,
    // };
    // let dummy_temtem_build = TemtemBuild {
    //     data: &dummy_temtem_data,
    // };
    // let dummy_temtem = TemtemInBattle {
    //     build: &dummy_temtem_build,
    //     stats: Stats {
    //         hp: 100,
    //     }
    // };

    // let tt_ac = TemtemAction::Swap(Temtem::Cerneaf);
    // let tt_ac = TemtemAction::Tech(Technique::Bush, Targeting::SingleOther(SingleOtherTarget::Ally));
    // let tt_ac = TemtemAction::Tech(Technique::Bush, Targeting::SingleOther(SingleOtherTarget::EnemyLeft));
    // let tt_ac = TemtemAction::Tech(Technique::Bush, Targeting::SingleOther(SingleOtherTarget::EnemyRight));
    // let tt_ac = TemtemAction::Tech(Technique::Bush, Targeting::SingleTeam(SingleTeamTarget::Own));
    // let tt_ac = TemtemAction::Rest;

    // let tech = Technique::A

    // let team = Rc::new(RefCell::new(TeamInBattle {
    //     temtems: [
    //         Rc::new(RefCell::new(TemtemInBattle {
    //             stats: Stats {
    //                 hp: 100,
    //             },
    //         })),
    //         Rc::new(RefCell::new(TemtemInBattle {
    //             stats: Stats {
    //                 hp: 100,
    //             },
    //         })),
    //         Rc::new(RefCell::new(TemtemInBattle {
    //             stats: Stats {
    //                 hp: 100,
    //             },
    //         })),
    //         Rc::new(RefCell::new(TemtemInBattle {
    //             stats: Stats {
    //                 hp: 100,
    //             },
    //         })),
    //         Rc::new(RefCell::new(TemtemInBattle {
    //             stats: Stats {
    //                 hp: 100,
    //             },
    //         })),
    //     ],
    //     left:  None,
    //     right: None,
    // }));

    // let ses: Vec<Box<dyn SideEffect>> = vec![
    //     Box::new(Swap {
    //         team:     team.clone(),
    //         temtem:   team.borrow().temtems[0].clone(),
    //         position: Position::Left,
    //     }),
    //     Box::new(Damage {
    //         value:  101,
    //         target: team.borrow().temtems[0].clone(),
    //     }),
    // ];

    // for se in ses.into_iter() {
    //     se.unwrap();
    // }

    // println!("{:#?}", team);

    // let state = Battle {
    //     team_a: TeamInBattle {
    //         temtems: [
    //             TemtemInBattle {
    //                 hp: 100,
    //             }
    //         ],
    //         left:  None,
    //         right: None,
    //     },
    //     team_b: TeamInBattle {
    //         temtems: [
    //             TemtemInBattle {
    //                 hp: 100,
    //             }
    //         ],
    //         left:  None,
    //         right: None,
    //     },
    // };

    // let turn_action = TurnAction {
    //     team_a: TeamAction {
    //         left:  TemtemAction::Rest,
    //         right: TemtemAction::Rest,
    //     },
    //     team_b: TeamAction {
    //         left:  TemtemAction::Rest,
    //         right: TemtemAction::Rest,
    //     },
    // };

    // let state = game_logic(state, turn_action);

    // let actor = MyActor {};
    // let state = Battle::Pre {
    //     team_a: CompetitiveTeam {
    //         temtems: [
    //             CompetitiveTemtem {},
    //         ],
    //     },
    //     team_b: CompetitiveTeam {
    //         temtems: [
    //             CompetitiveTemtem {},
    //         ],
    //     },
    // };

    // let controller = Controller::new(state, actor);

    // let state = controller.run();

    // println!("state = {:?}", state);
}
