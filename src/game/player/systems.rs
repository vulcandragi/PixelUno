use std::collections::VecDeque;

use bevy::{
    ecs::{
        hierarchy::Children,
        query::{Changed, With},
        relationship::RelationshipSourceCollection,
        system::{Commands, Query, ResMut},
    },
    math::Vec3,
    scene::CommandsSceneExt,
    state::commands::CommandsStatesExt,
    transform::components::Transform,
};
use rand::seq::SliceRandom;

use crate::game::{
    card::components::Card,
    components::{GameManager, GamePhase},
    player::{
        components::{Player, PlayerType},
        spawn_local_player, spawn_pc_player,
    },
};

type OrgazinePlayerHandPlayerQuery<'w, 's> =
    Query<'w, 's, (&'static PlayerType, &'static Children), (With<Player>, Changed<Children>)>;

pub fn register_players(mut commands: Commands, mut game_manager: ResMut<GameManager>) {
    let mut players = Vec::new();

    players.add(
        commands
            .spawn_scene(spawn_local_player("Guest".into()))
            .id(),
    );
    for i in 0..3 {
        players.add(commands.spawn_scene(spawn_pc_player(i)).id());
    }

    let mut rng = rand::rng();
    players.shuffle(&mut rng);

    game_manager.players = VecDeque::from(players);

    

    commands.set_state(GamePhase::DistributeCards);
}

pub fn orgazine_player_hand(
    players: OrgazinePlayerHandPlayerQuery,
    mut cards: Query<&mut Transform, With<Card>>,
) {
    for (player_type, card_entities) in players.iter() {
        if *player_type != PlayerType::Local {
            continue;
        }

        let card_count = card_entities.iter().count();
        let off_set = ((card_count as f32 * 60.) / 2.) + 30.;

        for (index, card_entity) in card_entities.iter().enumerate() {
            let Ok(mut transform) = cards.get_mut(*card_entity) else {
                continue;
            };

            transform.translation = Vec3 {
                x: (-off_set) + (60. * (index + 1) as f32),
                z: index as f32,
                ..Default::default()
            };
        }
    }
}
