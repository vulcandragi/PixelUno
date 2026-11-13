use bevy::{
    ecs::{
        entity::Entity,
        event::Event,
        observer::On,
        query::With,
        system::{Commands, Query, Res, ResMut, Single},
    },
    log::{info, warn},
    scene::CommandsSceneExt,
    state::{commands::CommandsStatesExt, state::State},
};

use crate::game::{
    card::{
        components::{Card, CardVisibility},
        spawn_card,
    },
    deck::components::Deck,
    discard_pile::components::DiscardPile,
    game_manager::components::{GameManager, GamePhase},
    player::components::{Player, PlayerType},
};

#[derive(Event, Debug)]
pub enum ActionIntent {
    Card { next_card: Entity },
    Buy,
}

pub fn on_action_intent(
    on: On<ActionIntent>,
    mut commands: Commands,
    mut game_manager: ResMut<GameManager>,
    game_phase: Res<State<GamePhase>>,
    mut discard_pile: Single<(Entity, &mut DiscardPile)>,
    mut deck: Single<&mut Deck>,
    cards: Query<&Card>,
    players: Query<&PlayerType, With<Player>>,
) {
    info!("{:?}", on);

    if *game_phase != GamePhase::WaitingForInput {
        return;
    }

    match *on {
        ActionIntent::Card {
            next_card: next_card_entity,
        } => {
            let (discard_pile_entity, mut discard_pile) = discard_pile.into_inner();
            let Some(card_entity) = discard_pile.last_card else {
                warn!("Last card not found");
                return;
            };

            let Ok(card) = cards.get(card_entity) else {
                warn!("Card not found");
                return;
            };

            let Ok(next_card) = cards.get(next_card_entity) else {
                warn!("Card not found");
                return;
            };

            if !card.check_card(next_card) {
                commands.set_state(GamePhase::WaitingForInput);
                return;
            }

            commands
                .entity(discard_pile_entity)
                .add_child(next_card_entity);
            discard_pile.last_card = Some(next_card_entity);
        }
        ActionIntent::Buy => {
            let Some(player_entity) = game_manager.current_player else {
                warn!("Current player not set");
                return;
            };

            let Ok(player_type) = players.get(player_entity) else {
                warn!("Player not found");
                return;
            };

            let Some(card) = deck.get_card() else {
                warn!("Deck is empty");
                return;
            };

            let card_visibility = match player_type {
                PlayerType::Local => CardVisibility::FaceUp,
                PlayerType::Pc { timer: _ } => CardVisibility::FaceDown,
            };

            let card_entity = commands.spawn_scene(spawn_card(card, card_visibility)).id();
            commands.entity(player_entity).add_child(card_entity);
        }
    };

    commands.set_state(GamePhase::PassingTurn);
}
