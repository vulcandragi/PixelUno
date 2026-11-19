use std::collections::VecDeque;

use bevy::{
    ecs::{
        entity::ContainsEntity,
        hierarchy::Children,
        query::{Changed, With},
        relationship::RelationshipSourceCollection,
        system::{Commands, Query, Res, ResMut, Single},
    },
    math::Vec3,
    scene::CommandsSceneExt,
    state::commands::CommandsStatesExt,
    time::Time,
    transform::components::Transform,
};
use rand::random_range;

use crate::game::{
    card::components::{Card, CardColor, CardSymbol},
    discard_pile::components::DiscardPile,
    game_manager::{
        components::{GameManager, GamePhase},
        events::ActionIntent,
    },
    player::{
        components::{Player, PlayerTurn, PlayerType},
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

type PcPlayerPlayPlayer<'w, 's> =
    Single<'w, 's, (&'static mut PlayerType, &'static Children), (With<Player>, With<PlayerTurn>)>;

pub fn pc_player_play(
    mut commands: Commands,
    time: Res<Time>,
    player: PcPlayerPlayPlayer,
    mut cards: Query<&mut Card>,
    discard_pile: Single<&DiscardPile>,
) {
    let (mut player_type, children) = player.into_inner();

    let PlayerType::Pc { timer } = &mut *player_type else {
        return;
    };

    timer.tick(time.delta());

    if !timer.just_finished() {
        return;
    }

    let Some(last_card_entity) = discard_pile.last_card else {
        return;
    };

    let Ok(last_card) = cards.get(last_card_entity) else {
        return;
    };

    let my_cards = children
        .iter()
        .filter_map(|entity| {
            let Ok(card) = cards.get(*entity) else {
                return None;
            };

            Some((entity, card))
        })
        .filter(|(_, card)| last_card.check_card(card))
        .map(|(entity, _)| entity)
        .collect::<Vec<_>>();

    if my_cards.is_empty() {
        commands.trigger(ActionIntent::Buy);
        return;
    }

    let Some(entity) = my_cards.get(random_range(0..my_cards.len())) else {
        return;
    };

    let Ok(mut card) = cards.get_mut(**entity) else {
        return;
    };

    if card.symbol == CardSymbol::Plus4 || card.symbol == CardSymbol::Color {
        card.color = CardColor::from(random_range(0..4));
    }

    commands.trigger(ActionIntent::Card {
        next_card: entity.entity(),
    });
}
