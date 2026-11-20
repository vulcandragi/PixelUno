use bevy::{
    ecs::system::{Commands, Query, Single},
    scene::CommandsSceneExt,
};

use crate::game::{
    card::{components::CardVisibility, spawn_card},
    deck::components::Deck,
    discard_pile::components::DiscardPile,
    game_manager::events::PassTurn,
    hand::components::Hand,
    player::components::{Player, PlayerType},
};

pub fn distribute_cards(
    mut commands: Commands,
    mut deck: Single<&mut Deck>,
    mut players: Query<(&Player, &PlayerType)>,
    mut discard_pile: Single<&mut DiscardPile>,
    mut hands: Query<&mut Hand>,
) {
    for _ in 0..7 {
        for (player, player_type) in players.iter_mut() {
            let Some(card) = deck.cards.pop_back() else {
                break;
            };

            let Ok(mut hand) = hands.get_mut(player.hand) else {
                continue;
            };

            let card_visibility = match player_type {
                PlayerType::Local => CardVisibility::FaceUp,
                PlayerType::Pc { timer: _ } => CardVisibility::FaceDown,
            };

            let card_entity = commands.spawn_scene(spawn_card(card, card_visibility)).id();
            commands.entity(player.hand).add_child(card_entity);

            hand.cards.push(card_entity);
        }
    }

    let Some(card) = deck.cards.pop_back() else {
        return;
    };

    discard_pile.last_card = Some(
        commands
            .spawn_scene(spawn_card(card, CardVisibility::FaceUp))
            .id(),
    );

    commands.trigger(PassTurn(1));
}
