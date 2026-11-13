use bevy::{
    asset::Handle,
    ecs::{component::Component, resource::Resource},
    image::{Image, TextureAtlasLayout},
    reflect::Reflect,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Component, Clone, Reflect, Debug)]
pub struct Card {
    pub color: CardColor,
    pub symbol: CardSymbol,
}

#[derive(Component, Clone, Default, Reflect, PartialEq)]
pub enum CardVisibility {
    FaceUp,
    #[default]
    FaceDown,
}

#[derive(Resource, AssetCollection)]
pub struct CardAssets {
    #[asset(path = "images/cards.png")]
    pub image: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 84, tile_size_y = 120, columns = 15, rows = 5))]
    pub atlas_layout: Handle<TextureAtlasLayout>,
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

impl Card {
    pub fn get_index(&self) -> usize {
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
}
