use bevy::{
    asset::Handle,
    ecs::{component::Component, entity::Entity, resource::Resource, template::FromTemplate},
    image::Image,
    reflect::Reflect,
    time::Timer,
};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Component, FromTemplate, Reflect)]
pub struct Player {
    pub name: String,
    pub hand: Entity,
}

#[derive(Component, FromTemplate, Reflect)]
pub struct PlayerName;

#[derive(Component, FromTemplate, Reflect)]
pub struct PlayerCardCount {
    pub hand: Entity,
}

#[derive(Resource, AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "images/player_plate.png")]
    pub plate: Handle<Image>,
}

#[derive(Component, Clone, Default, Reflect, PartialEq)]
pub enum PlayerType {
    #[default]
    Local,
    Pc {
        timer: Timer,
    },
}

#[derive(Component, Reflect)]
pub struct PlayerTurn;
