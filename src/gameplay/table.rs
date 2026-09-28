use bevy::{
    asset::Handle,
    ecs::{
        component::Component,
        name::Name,
        observer::On,
        resource::Resource,
        system::{Commands, Res},
    },
    image::Image,
    sprite::Sprite,
    transform::components::Transform,
};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::events::Spawn;

#[derive(Component)]
pub struct Table;

#[derive(Resource, AssetCollection)]
pub struct TableAssets {
    #[asset(path = "images/background.webp")]
    pub image: Handle<Image>,
}

impl Table {
    pub fn on_spawn(_: On<Spawn<Table>>, mut commands: Commands, assets: Res<TableAssets>) {
        commands
            .spawn(Table)
            .insert(Name("Table".into()))
            .insert(Transform::from_xyz(0., 0., -100.))
            .insert(Sprite {
                image: assets.image.clone(),
                ..Default::default()
            });
    }
}
