use std::collections::VecDeque;

use bevy::{
    asset::Handle,
    ecs::{
        component::Component,
        entity::Entity,
        observer::On,
        query::With,
        resource::Resource,
        system::{Commands, Query, Res, Single},
    },
    image::Image,
    picking::{
        Pickable,
        events::{Click, Enter, Out, Pointer},
    },
    sprite::Sprite,
    transform::components::Transform,
    window::{CursorIcon, SystemCursorIcon, Window},
};
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::seq::SliceRandom;

use crate::{
    events::Spawn,
    gameplay::{
        card::{Card, CardColor, CardSpawnData, CardSymbol},
        hand::Hand,
    },
};

#[derive(Component)]
pub struct Deck {
    pub cards: VecDeque<Card>,
}

#[derive(Resource, AssetCollection)]
pub struct DeckAssets {
    #[asset(path = "images/deck.png")]
    pub image: Handle<Image>,
}

impl Deck {
    pub fn on_spawn(_: On<Spawn<Deck>>, mut commands: Commands, assets: Res<DeckAssets>) {
        let deck = Deck {
            cards: Self::generate_deck(),
        };

        commands
            .spawn(deck)
            .insert(Transform::from_xyz(500.0, 0.0, 0.0))
            .insert(Sprite {
                image: assets.image.clone(),
                ..Default::default()
            })
            .insert(Pickable::default())
            .observe(Self::on_hover_enter)
            .observe(Self::on_hover_out)
            .observe(Self::on_click);
    }

    fn generate_deck() -> VecDeque<Card> {
        let mut cards = Vec::new();

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

        let mut rng = rand::rng();
        cards.shuffle(&mut rng);

        VecDeque::from(cards)
    }

    fn on_hover_enter(
        _: On<Pointer<Enter>>,
        mut commands: Commands,
        window: Single<Entity, With<Window>>,
    ) {
        commands
            .entity(*window)
            .insert(CursorIcon::from(SystemCursorIcon::Pointer));
    }

    fn on_hover_out(
        _: On<Pointer<Out>>,
        mut commands: Commands,
        window: Single<Entity, With<Window>>,
    ) {
        commands
            .entity(*window)
            .insert(CursorIcon::from(SystemCursorIcon::Default));
    }

    fn on_click(
        _: On<Pointer<Click>>,
        mut commands: Commands,
        mut deck: Single<&mut Deck>,
        mut hand_query: Query<Entity, With<Hand>>,
    ) {
        let Some(hand_entity) = hand_query.iter_mut().next() else {
            return;
        };

        let Some(card) = deck.cards.pop_back() else {
            return;
        };

        commands.trigger(
            Spawn::<Card, CardSpawnData>::default()
                .with_data(CardSpawnData {
                    color: card.color,
                    symbol: card.symbol,
                })
                .with_parent(hand_entity),
        );
    }
}
