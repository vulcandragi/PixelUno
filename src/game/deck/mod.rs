use bevy::{
    ecs::template::template,
    picking::Pickable,
    scene::{Scene, bsn, on},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::game::deck::{
    components::{Deck, DeckAssets},
    events::on_player_click_discard_pile,
};

pub mod components;
pub mod events;
pub mod systems;

pub fn spawn_deck() -> impl Scene {
    bsn! {
        #Deck
        Deck::new()
        Transform::from_xyz(500.0, 0.0, 0.0)
        template(|context| {
            let assets = context.resource::<DeckAssets>();
            Ok(Sprite {
                image: assets.image.clone(),
                ..Default::default()
            })
        })
        Pickable
        on(on_player_click_discard_pile)
    }
}
