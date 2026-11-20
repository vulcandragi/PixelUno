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
    deck::components::Deck,
    discard_pile::components::DiscardPile,
    game_manager::{
        components::{GameManager, GamePhase},
        events::ActionIntent,
    },
    hand::components::Hand,
    player::{
        components::{Player, PlayerTurn, PlayerType},
        spawn_local_player, spawn_pc_player,
    },
};

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

pub fn update_player_card_count(
    players: Query<&Children, (With<Player>, Changed<Children>)>,
    cards: Query<&Card>,
) {
    for card_entities in players.iter() {}
}

pub fn pc_player_play(
    mut commands: Commands,
    time: Res<Time>,
    player: Single<(&Player, &mut PlayerType), With<PlayerTurn>>,
    hands: Query<&Hand>,
    mut cards: Query<&mut Card>,
    discard_pile: Single<&DiscardPile>,
    deck: Single<&Deck>,
) {
    let (player, mut player_type) = player.into_inner();

    let PlayerType::Pc { timer } = &mut *player_type else {
        return;
    };

    timer.tick(time.delta());

    if !timer.just_finished() {
        return;
    }

    let Ok(hand) = hands.get(player.hand) else {
        return;
    };

    let Some(last_card_entity) = discard_pile.last_card else {
        return;
    };

    let Ok(last_card) = cards.get(last_card_entity) else {
        return;
    };

    let my_cards = hand
        .cards
        .iter()
        .filter_map(|entity| {
            let Ok(card) = cards.get(entity) else {
                return None;
            };

            Some((entity, card))
        })
        .filter(|(_, card)| {
            if deck.buy_amount.is_some() {
                last_card.check_plus(card)
            } else {
                last_card.check_card(card)
            }
        })
        .map(|(entity, _)| entity)
        .collect::<Vec<_>>();

    if my_cards.is_empty() {
        commands.trigger(ActionIntent::Buy);
        return;
    }

    let Some(entity) = my_cards.get(random_range(0..my_cards.len())) else {
        return;
    };

    let Ok(mut card) = cards.get_mut(*entity) else {
        return;
    };

    if card.symbol == CardSymbol::Plus4 || card.symbol == CardSymbol::Color {
        card.color = CardColor::from(random_range(0..4));
    }

    commands.trigger(ActionIntent::Card {
        next_card: entity.entity(),
    });
}
