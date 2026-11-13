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

use crate::game::{
    card::components::Card,
    game_manager::components::{GameManager, GamePhase},
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

    game_manager.players = VecDeque::from(players);

    commands.set_state(GamePhase::DistributeCards);
}

pub fn orgazine_player_hand(
    players: OrgazinePlayerHandPlayerQuery,
    mut cards: Query<(&Card, &mut Transform)>,
) {
    for (player_type, card_entities) in players.iter() {
        if *player_type != PlayerType::Local {
            continue;
        }

        let card_count = card_entities.iter().count();
        let off_set = ((card_count as f32 * 60.) / 2.) + 30.;

        let mut sorted_cards = card_entities.iter().copied().collect::<Vec<_>>();

        sorted_cards
            .sort_by_key(|entity| cards.get(*entity).map_or(0, |(card, _)| card.get_index()));

        for (index, card_entity) in sorted_cards.iter().enumerate() {
            let Ok((card, mut transform)) = cards.get_mut(card_entity) else {
                continue;
            };

            transform.translation = Vec3 {
                x: (-off_set) + (60. * (index + 1) as f32),
                z: card.get_index() as f32,
                ..Default::default()
            };
        }
    }
}
