use std::collections::VecDeque;

use bevy::{
    asset::Handle,
    ecs::{component::Component, resource::Resource},
    image::Image,
    reflect::Reflect,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::seq::SliceRandom;

use crate::game::card::components::{Card, CardColor, CardSymbol};

#[derive(Component, Clone, Default, Reflect)]
pub struct Deck {
    pub cards: VecDeque<Card>,
}

#[derive(Resource, AssetCollection)]
pub struct DeckAssets {
    #[asset(path = "images/deck.png")]
    pub image: Handle<Image>,
}

pub fn generate_deck() -> VecDeque<Card> {
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
