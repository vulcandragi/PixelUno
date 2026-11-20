use bevy::{
    camera::visibility::InheritedVisibility,
    ecs::{hierarchy::Children, template::template},
    math::{Vec2, Vec3},
    scene::{Scene, bsn, on},
    sprite::{BorderRect, SliceScaleMode, Sprite, SpriteImageMode, Text2d, TextureSlicer},
    time::{Timer, TimerMode},
    transform::components::Transform,
};
use rand::random_range;

use crate::game::{
    hand::spawn_hand,
    player::{
        components::{Player, PlayerAssets, PlayerCardCount, PlayerName, PlayerType},
        events::on_player_click_card,
    },
};

pub mod components;
pub mod events;
pub mod systems;

pub fn spawn_local_player(name: String) -> impl Scene {
    bsn! {
        #Player
        Player {
            name,
            hand: #Hand
        }
        PlayerType::Local
        Transform::from_xyz(0., -220., 0.)
        Children [
            #Hand
            @spawn_hand(#Player)
        ]
        on(on_player_click_card)
    }
}

pub fn spawn_pc_player(number: i32) -> impl Scene {
    let random_time = random_range(1.0..3.0);
    let timer = Timer::from_seconds(random_time, TimerMode::Repeating);

    let x = (480 * number) as f32 - 480.;
    let name = format!("Pc {number}");

    bsn! {
        #Player
        Player {
            name: {name.clone()},
            hand: #Hand
        }
        PlayerType::Pc { timer: timer }
        Transform::from_xyz(x, 200., 0.)
        InheritedVisibility
        Children [
            #Hand
            @spawn_hand(#Player)
            --
            #PlayerName
            PlayerName
            @spawn_plate(
                name,
                Vec3::new(0., 50., 1000.),
                Vec2::new(100., 30.)
            )
            --
            #PlayerCardCount
            PlayerCardCount
            @spawn_plate(
                "0".into(),
                Vec3::new(20., -40., 1000.),
                Vec2::new(30., 30.)
            )
        ]
    }
}

fn spawn_plate(text: String, offset: Vec3, size: Vec2) -> impl Scene {
    bsn! {
        Transform::from_translation(offset)
        template(move |context| {
            let assets = context.resource::<PlayerAssets>();
            let slicer = TextureSlicer {
                border: BorderRect::all(10.),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                ..Default::default()
            };

            Ok(Sprite {
                image: assets.plate.clone(),
                image_mode: SpriteImageMode::Sliced(slicer),
                custom_size: Some(size),
                ..Default::default()
            })
        })
        Children [
            Text2d::new(text)
        ]
    }
}
