use bevy::scene::{Scene, bsn};

use crate::game::discard_pile::components::DiscardPile;

pub mod components;
pub mod systems;

pub fn spawn_discard_pile() -> impl Scene {
    bsn! {
        #DiscardPile
        DiscardPile
    }
}
