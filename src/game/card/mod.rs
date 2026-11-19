use bevy::{
    ecs::template::template,
    image::TextureAtlas,
    math::Vec3,
    picking::Pickable,
    scene::{Scene, bsn},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::game::card::components::{Card, CardAssets, CardVisibility};

pub mod components;
pub mod systems;

pub fn spawn_card(card: Card, card_visibility: CardVisibility) -> impl Scene {
    let atlas_index = match card_visibility {
        CardVisibility::FaceUp => card.get_index(),
        CardVisibility::FaceDown => 62,
    };

    bsn! {
        #Card
        card
        card_visibility
        Transform::from_scale(Vec3 {
            x: 0.7,
            y: 0.7,
            z: 0.7,
        })
        template(move |context| {
            let assets = context.resource::<CardAssets>();
            Ok(Sprite {
                image: assets.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.atlas_layout.clone(),
                    index: atlas_index
                }),
                ..Default::default()
            })
        })
        Pickable
    }
}
