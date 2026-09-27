use bevy::{
    asset::Handle,
    ecs::{
        component::Component,
        entity::Entity,
        observer::On,
        query::With,
        resource::Resource,
        system::{Commands, Res, Single},
    },
    image::Image,
    picking::{
        Pickable,
        events::{Enter, Out, Pointer},
    },
    sprite::Sprite,
    transform::components::Transform,
    window::{CursorIcon, SystemCursorIcon, Window},
};
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::seq::SliceRandom;

use crate::{
    events::Spawn,
    gameplay::card::{Card, CardColor, CardSymbol},
};

#[derive(Component, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

#[derive(Resource, AssetCollection)]
pub struct DeckAssets {
    #[asset(path = "images/deck.png")]
    pub image: Handle<Image>,
}

impl Deck {
    pub fn on_spawn(_: On<Spawn<Deck>>, mut commands: Commands, assets: Res<DeckAssets>) {
        let mut deck = Deck::default();
        deck.generate_deck();

        commands
            .spawn(deck)
            .insert(Transform::from_xyz(500.0, 0.0, 0.0))
            .insert(Sprite {
                image: assets.image.clone(),
                ..Default::default()
            })
            .insert(Pickable::default())
            .observe(Self::on_hover_enter)
            .observe(Self::on_hover_out);
    }

    fn generate_deck(&mut self) {
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

        self.cards = cards
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
}
