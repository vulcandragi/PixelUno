use crate::plugins::card::{AddCard, Card, CardColor, CardSymbol};
use crate::plugins::game::GameState;
use crate::plugins::load::LoadState;
use crate::plugins::player::Player;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use rand::prelude::*;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(LoadState::Loading).load_collection::<DeckAssets>(),
        )
        .add_systems(OnEnter(GameState::Start), create_deck)
        .add_systems(Update, dealing.run_if(on_message::<DeckLoadMessage>))
        .add_message::<DeckLoadMessage>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct DeckAssets {
    #[asset(path = "images/deck.png")]
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct Deck {
    pub cards: Vec<Card>,
}

#[derive(Message)]
pub struct DeckLoadMessage;

fn create_deck(
    mut commands: Commands,
    assets: Res<DeckAssets>,
    mut message_writer: MessageWriter<DeckLoadMessage>,
) {
    commands.spawn((
        Name::new("Deck"),
        Deck {
            cards: generate_deck(1),
        },
        Sprite {
            image: assets.image.clone(),
            ..default()
        },
        Transform::from_xyz(500.0, 0.0, 0.0).with_scale(Vec3::splat(1.5)),
    ));
    message_writer.write(DeckLoadMessage);
}

fn generate_deck(amount: u8) -> Vec<Card> {
    let mut cards = Vec::new();

    for _ in 0..amount {
        for color in 0..4 {
            cards.push(Card {
                color: CardColor::from(color),
                symbol: CardSymbol::Zero,
            });

            for symbol in 1..=12 {
                for _ in 0..2 {
                    cards.push(Card {
                        color: CardColor::from(color),
                        symbol: CardSymbol::from(symbol),
                    });
                }
            }
        }
        for symbol in 13..=14 {
            for _ in 0..4 {
                cards.push(Card {
                    color: CardColor::Black,
                    symbol: CardSymbol::from(symbol),
                });
            }
        }
    }

    let mut rng = rand::rng();
    cards.shuffle(&mut rng);

    cards
}

fn dealing(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    mut deck: Single<&mut Deck>,
) {
    for player in players.iter() {
        for _ in 0..7 {
            if let Some(card) = deck.cards.pop() {
                commands.trigger(AddCard {
                    entity: player,
                    card,
                })
            }
        }
    }
}
