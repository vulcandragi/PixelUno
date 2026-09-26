use bevy::{
    asset::Handle,
    ecs::{component::Component, resource::Resource},
    image::{Image, TextureAtlasLayout},
    reflect::Reflect,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Component)]
pub struct Card {
    pub color: CardColor,
    pub symbol: CardSymbol,
}

#[derive(Resource, AssetCollection)]
pub struct CardAssets {
    #[asset(texture_atlas_layout(tile_size_x = 84, tile_size_y = 120, columns = 15, rows = 5))]
    pub cards: Handle<TextureAtlasLayout>,
    #[asset(path = "images/cards.png")]
    pub cards_texture: Handle<Image>,
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
