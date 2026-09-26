use bevy::app::App;
use bevy::prelude::*;

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(FixedUpdate, position_cards);
    }
}

#[derive(Component)]
pub struct Hand;