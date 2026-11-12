use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Single},
    },
    scene::CommandsSceneExt,
    state::commands::CommandsStatesExt,
};

use crate::game::{
    card::{components::CardVisibility, spawn_card},
    components::GamePhase,
    deck::components::Deck,
    discard_pile::components::DiscardPile,
    player::components::{Player, PlayerType},
};

pub fn distribute_cards(
    mut commands: Commands,
    mut deck: Single<&mut Deck>,
    mut players: Query<(Entity, &mut Player, &PlayerType)>,
    mut discard_pile: Single<&mut DiscardPile>,
) {
    for _ in 0..7 {
        for (player_entity, mut player, player_type) in players.iter_mut() {
            let Some(card) = deck.cards.pop_back() else {
                break;
            };

            let card_visibility = match player_type {
                PlayerType::Local => CardVisibility::FaceUp,
                PlayerType::Pc { timer: _ } => CardVisibility::FaceDown,
            };

            let card_entity = commands.spawn_scene(spawn_card(card, card_visibility)).id();
            commands.entity(player_entity).add_child(card_entity);

            player.cards.push(card_entity);
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

    commands.set_state(GamePhase::PassingTurn);
}
