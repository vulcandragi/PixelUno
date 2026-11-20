use bevy::{
    ecs::{
        entity::Entity,
        event::Event,
        observer::On,
        relationship::RelationshipSourceCollection,
        system::{Commands, Query, Res, ResMut, Single, SystemParam},
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
    hand::components::Hand,
    player::components::{Player, PlayerType},
};

#[derive(Event, Debug)]
pub enum ActionIntent {
    Card { next_card: Entity },
    Buy,
}

#[derive(SystemParam)]
pub struct OnActionIntentParam<'w, 's> {
    pub game_manager: ResMut<'w, GameManager>,
    pub game_phase: Res<'w, State<GamePhase>>,
    pub discard_pile: Single<'w, 's, (Entity, &'static mut DiscardPile)>,
    pub deck: Single<'w, 's, &'static mut Deck>,
    pub cards: Query<'w, 's, &'static Card>,
    pub players: Query<'w, 's, (&'static Player, &'static PlayerType)>,
    pub hands: Query<'w, 's, &'static mut Hand>,
}

pub fn on_action_intent(
    on: On<ActionIntent>,
    mut commands: Commands,
    mut param: OnActionIntentParam,
) {
    info!("{:?}", on);

    if *param.game_phase != GamePhase::WaitingForInput
        && *param.game_phase != GamePhase::WaitingColorSelect
    {
        return;
    }

    let Some(player_entity) = param.game_manager.current_player else {
        warn!("Current player not set");
        return;
    };

    let Ok((player, player_type)) = param.players.get(player_entity) else {
        warn!("Player not found");
        return;
    };

    let Ok(mut hand) = param.hands.get_mut(player.hand) else {
        warn!("Hand not found");
        return;
    };

    match *on {
        ActionIntent::Card {
            next_card: next_card_entity,
        } => {
            let (discard_pile_entity, mut discard_pile) = param.discard_pile.into_inner();
            let Some(card_entity) = discard_pile.last_card else {
                warn!("Last card not found");
                return;
            };

            let Ok(card) = param.cards.get(card_entity) else {
                warn!("Card not found");
                return;
            };

            let Ok(next_card) = param.cards.get(next_card_entity) else {
                warn!("Card not found");
                return;
            };

            info!("{next_card:?}");

            if !card.check_card(next_card) {
                commands.set_state(GamePhase::WaitingForInput);
                return;
            }

            if param.deck.buy_amount.is_some() && !card.check_plus(next_card) {
                commands.set_state(GamePhase::WaitingForInput);
                return;
            }

            if next_card.is_plus() {
                param.deck.buy_amount = Some(param.deck.buy_amount.unwrap_or(0) + next_card.plus())
            }

            commands
                .entity(discard_pile_entity)
                .add_child(next_card_entity);
            discard_pile.last_card = Some(next_card_entity);

            RelationshipSourceCollection::remove(&mut hand.cards, next_card_entity);
        }
        ActionIntent::Buy => {
            let amount = param.deck.buy_amount.unwrap_or(1);

            for _ in 0..amount {
                let Some(card) = param.deck.get_card() else {
                    warn!("Deck is empty");
                    return;
                };

                info!("{card:?}");

                let card_visibility = match player_type {
                    PlayerType::Local => CardVisibility::FaceUp,
                    PlayerType::Pc { timer: _ } => CardVisibility::FaceDown,
                };

                let card_entity = commands.spawn_scene(spawn_card(card, card_visibility)).id();
                commands.entity(player.hand).add_child(card_entity);

                hand.cards.push(card_entity);
            }

            param.deck.buy_amount = None;
        }
    };

    commands.set_state(GamePhase::PassingTurn);
}
