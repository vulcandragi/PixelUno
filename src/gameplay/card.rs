use bevy::{
    asset::Handle,
    ecs::{
        component::Component,
        entity::Entity,
        event::EntityEvent,
        name::Name,
        observer::On,
        resource::Resource,
        system::{Commands, Res},
    },
    image::{Image, TextureAtlas, TextureAtlasLayout},
    picking::{
        Pickable,
        events::{Click, Pointer},
    },
    reflect::Reflect,
    sprite::Sprite,
    transform::components::Transform,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::events::Spawn;

#[derive(Component, Clone, Debug)]
pub struct Card {
    pub color: CardColor,
    pub symbol: CardSymbol,
}

#[derive(Default)]
pub struct CardSpawnData {
    pub color: CardColor,
    pub symbol: CardSymbol,
}

#[derive(Resource, AssetCollection)]
pub struct CardAssets {
    #[asset(texture_atlas_layout(tile_size_x = 84, tile_size_y = 120, columns = 15, rows = 5))]
    pub atlas: Handle<TextureAtlasLayout>,
    #[asset(path = "images/cards.png")]
    pub image: Handle<Image>,
}

#[derive(IntoPrimitive, FromPrimitive, Reflect, Copy, Clone, Debug, Default, PartialEq)]
#[repr(u8)]
pub enum CardColor {
    Blue = 0,
    Yellow,
    Red,
    Green,
    Black,
    #[default]
    None,
}

#[derive(IntoPrimitive, FromPrimitive, Reflect, Copy, Clone, Debug, Default, PartialEq)]
#[repr(u8)]
pub enum CardSymbol {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Block,
    Reverse,
    Plus2,
    Color,
    Plus4,
    #[default]
    None,
}

#[derive(EntityEvent)]
#[entity_event(propagate)]
pub struct AddCard {
    pub entity: Entity,
    pub card: Entity,
}

impl Card {
    pub fn on_spawn(
        event: On<Spawn<Card, CardSpawnData>>,
        mut commands: Commands,
        assests: Res<CardAssets>,
    ) {
        let card = Card {
            color: event.data.color,
            symbol: event.data.symbol,
        };
        let atlas_index = card.get_order();

        let entity = commands
            .spawn(card)
            .insert(Name("Card".into()))
            .insert(Sprite {
                image: assests.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: atlas_index,
                    layout: assests.atlas.clone(),
                }),
                ..Default::default()
            })
            .insert(Transform::from_xyz(0., 0., 0.))
            .insert(Pickable::default())
            .id();

        if let Some(parent) = event.parent {
            commands.entity(entity).observe(Self::on_click);
            commands.entity(parent).add_child(entity);
            commands.trigger(AddCard {
                entity: parent,
                card: entity,
            });
        }
    }

    pub fn get_order(&self) -> usize {
        if self.color == CardColor::Black {
            return match self.symbol {
                CardSymbol::Color => 60usize,
                CardSymbol::Plus4 => 61usize,
                _ => 62usize,
            };
        }

        if self.symbol == CardSymbol::None || self.color == CardColor::None {
            63usize
        } else {
            ((u8::from(self.color)) * 15 + u8::from(self.symbol)) as usize
        }
    }

    pub fn check_card(&self, next_card: &Card) -> bool {
        if next_card.color == CardColor::None || next_card.symbol == CardSymbol::None {
            return false;
        }

        if self.color == next_card.color {
            return true;
        }

        if self.symbol == next_card.symbol {
            return true;
        }

        if next_card.color == CardColor::Black {
            return true;
        }

        false
    }

    pub fn on_click(mut event: On<Pointer<Click>>) {
        event.propagate(true);
    }
}
