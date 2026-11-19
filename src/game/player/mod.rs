use bevy::{
    camera::visibility::InheritedVisibility,
    ecs::{hierarchy::Children, name::Name, template::template},
    scene::{Scene, bsn, on},
    sprite::{Sprite, Text2d},
    time::{Timer, TimerMode},
    transform::components::Transform,
};
use rand::random_range;

use crate::game::player::{
    components::{Player, PlayerAssets, PlayerName, PlayerType},
    events::on_player_click_card,
};

pub mod components;
pub mod events;
pub mod systems;

pub fn spawn_local_player(name: String) -> impl Scene {
    bsn! {
        Player
        PlayerType::Local
        Name(name)
        Transform::from_xyz(0., -220., 0.)
        on(on_player_click_card)
    }
}

pub fn spawn_pc_player(number: i32) -> impl Scene {
    let random_time = random_range(1.0..3.0);
    let timer = Timer::from_seconds(random_time, TimerMode::Repeating);

    let x = (480 * number) as f32 - 480.;
    let name = format!("Pc {number}");

    bsn! {
        Player
        PlayerType::Pc { timer: timer }
        Name({name.clone()})
        Transform::from_xyz(x, 200., 0.)
        InheritedVisibility
        Children [
            PlayerName
            Transform::from_xyz(0., 50., 1000.)
            template(|context| {
                let assets = context.resource::<PlayerAssets>();

                Ok(Sprite {
                    image: assets.menu_name.clone(),
                    ..Default::default()
                })
            })
            Children [
                Text2d::new(name)
            ]
        ]
    }
}
