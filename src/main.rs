mod entities;
mod plugins;
mod systems;

use crate::plugins::game::GamePlugin;
use crate::plugins::load::LoadPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pixel Uno".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
            EguiPlugin::default(),
            WorldInspectorPlugin::default(),
            LoadPlugin,
            GamePlugin,
        ))
        // .insert_state(GameState::BootingApp)
        // .add_systems(OnEnter(GameState::StartScreen), startup)
        .run();
}

// pub fn startup(mut commands: Commands, assets: Res<CardAssets>) {
//     commands.insert_resource(ClearColor(Color::Srgba(Srgba::hex("9c6024").unwrap())));
//     commands.spawn((
//         Camera2d,
//         Projection::from(OrthographicProjection {
//             scaling_mode: FixedVertical {
//                 viewport_height: 1080.0,
//             },
//             ..OrthographicProjection::default_2d()
//         }),
//     ));
//     commands.spawn((
//         Sprite::from_atlas_image(
//             assets.cards_texture.clone(),
//             TextureAtlas {
//                 layout: assets.cards.clone(),
//                 index: 0,
//             },
//         ),
//         Card,
//     ));
// }
//
// #[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
// pub enum GameState {
//     BootingApp,
//     StartScreen,
//     ErrorScreen,
// }
