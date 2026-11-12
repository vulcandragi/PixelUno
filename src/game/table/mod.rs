use bevy::{
    ecs::template::template,
    scene::{Scene, bsn},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::game::table::components::{Table, TableAssets};

pub mod components;

pub fn spawn_table() -> impl Scene {
    bsn! {
        #Table
        Table
        Transform::from_xyz(0., 0., -1.)
        template(|context| {
            let assets = context.resource::<TableAssets>();
            Ok(Sprite {
                image: assets.image.clone(),
                ..Default::default()
            })
        })
    }
}
