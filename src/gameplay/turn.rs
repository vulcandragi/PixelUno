use std::collections::VecDeque;

use bevy::{
    ecs::{entity::Entity, resource::Resource},
    state::state::States,
};

#[derive(Resource, Default)]
pub struct TurnManager {
    pub players: VecDeque<Entity>,
    pub direction: TurnDirection,
}

#[derive(Default)]
pub enum TurnDirection {
    #[default]
    Clockwise,
    CounterClockwise,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum TurnPhase {
    #[default]
    WaitingForInput,
    EvaluatingAction,
    PassingTurn,
}

impl TurnManager {}
