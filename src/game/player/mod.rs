use bevy::{
    ecs::name::Name,
    scene::{Scene, bsn, on},
    time::{Timer, TimerMode},
    transform::components::Transform,
};
use rand::random_range;

use crate::game::player::{
    components::{Player, PlayerType},
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
        Transform::from_xyz(0., -200., 0.)
        on(on_player_click_card)
    }
}

pub fn spawn_pc_player(number: i32) -> impl Scene {
    let random_time = random_range(1.0..3.0);
    let timer = Timer::from_seconds(random_time, TimerMode::Repeating);

    let x = (480 * number) as f32 - 480.;

    bsn! {
        Player
        PlayerType::Pc { timer: timer }
        Name(format!("Pc {number}"))
        Transform::from_xyz(x, 200., 0.)
    }
}
