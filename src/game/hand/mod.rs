use bevy::{
    ecs::template::EntityTemplate,
    scene::{Scene, bsn},
    transform::components::Transform,
};

use crate::game::hand::components::Hand;

pub mod components;
pub mod systems;

pub fn spawn_hand(player: impl Into<EntityTemplate>) -> impl Scene {
    bsn! {
        #Hand
        Hand {
            owner: player,
            cards: vec![],
        }
        Transform
    }
}
