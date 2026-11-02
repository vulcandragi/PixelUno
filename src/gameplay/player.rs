use bevy::ecs::{
    component::Component,
    entity::{ContainsEntity, Entity},
    observer::On,
    query::With,
    system::{Commands, Single},
};

use crate::{events::Spawn, gameplay::turn::TurnManager};

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn on_spawn(
        _: On<Spawn<Player>>,
        mut commands: Commands,
        turn: Single<Entity, With<TurnManager>>,
    ) {
        let entity = commands.spawn(Player).id();

        commands.entity(turn.entity()).add_child(entity);
    }
}
