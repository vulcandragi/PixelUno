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
    WaitingForInput,
    EvaluatingAction,
}

#[derive(Resource, Default, Reflect)]
#[reflect(ReflectResource)]
pub struct GameManager {
    pub players: VecDeque<Entity>,
    pub direction: GameDirection,
    pub current_player: Option<Entity>,
}

#[derive(Default, Clone, Reflect)]
pub enum GameDirection {
    #[default]
    Clockwise,
    CounterClockwise,
}

impl GameManager {
    pub fn peek_next_player(&self) -> Option<Entity> {
        let Some(current) = self.current_player else {
            return self.players.front().copied();
        };

        let current_index = self.players.iter().position(|&e| e == current)?;

        let len = self.players.len();
        if len == 0 {
            return None;
        }

        let next_index = match self.direction {
            GameDirection::Clockwise => (current_index + 1) % len,
            GameDirection::CounterClockwise => (current_index + len - 1) % len,
        };

        self.players.get(next_index).copied()
    }

    pub fn advance_turn(&mut self) -> Option<Entity> {
        let next = self.peek_next_player()?;
        self.current_player = Some(next);
        Some(next)
    }
}
