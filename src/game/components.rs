use std::collections::VecDeque;

use bevy::{
    ecs::{entity::Entity, reflect::ReflectResource, resource::Resource},
    reflect::Reflect,
    state::state::States,
};

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GamePhase {
    #[default]
    None,
    RegisterPlayers,
    DistributeCards,
    PassingTurn,
}

#[derive(Resource, Default, Reflect)]
#[reflect(ReflectResource)]
pub struct GameManager {
    pub players: VecDeque<Entity>,
    pub direction: GameDirection,
}

#[derive(Default, Clone, Reflect)]
pub enum GameDirection {
    #[default]
    Clockwise,
    CounterClockwise,
}
