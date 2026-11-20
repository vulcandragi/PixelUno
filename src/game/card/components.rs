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

        if next_card.symbol == CardSymbol::Plus4 || next_card.symbol == CardSymbol::Color {
            return true;
        }

        if self.color == next_card.color {
            return true;
        }

        if self.symbol == next_card.symbol {
            return true;
        }

        false
    }

    pub fn check_plus(&self, next_card: &Card) -> bool {
        if !next_card.is_plus() || !self.is_plus() {
            return false;
        }

        u8::from(self.symbol) <= u8::from(next_card.symbol)
    }

    pub fn is_plus(&self) -> bool {
        self.symbol == CardSymbol::Plus2 || self.symbol == CardSymbol::Plus4
    }

    pub fn plus(&self) -> u8 {
        match self.symbol {
            CardSymbol::Plus2 => 2,
            CardSymbol::Plus4 => 4,
            _ => 0,
        }
    }
}

#[test]
fn feature_check_card() {
    let last_card = Card {
        color: CardColor::Yellow,
        symbol: CardSymbol::Four,
    };

    let next_card = Card {
        color: CardColor::Green,
        symbol: CardSymbol::Plus4,
    };

    assert!(last_card.check_card(&next_card))
}
