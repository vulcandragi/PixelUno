use bevy::{
    ecs::template::template,
    picking::Pickable,
    scene::{Scene, bsn},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::game::deck::components::{Deck, DeckAssets, generate_deck};

pub mod components;
pub mod systems;

pub fn spawn_deck() -> impl Scene {
    bsn! {
        #Deck
        Deck { cards: generate_deck() }
        Transform::from_xyz(500.0, 0.0, 0.0)
        template(|context| {
            let assets = context.resource::<DeckAssets>();
            Ok(Sprite {
                image: assets.image.clone(),
                ..Default::default()
            })
        })
        Pickable
    }
}
