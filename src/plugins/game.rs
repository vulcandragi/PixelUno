use crate::plugins::load::LoadCompleteMessage;
use bevy::camera::ScalingMode::FixedVertical;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pixel Uno".to_string(),
                    ..default()
                }),
                ..default()
            }),))
            .insert_state(GameState::None)
            .add_systems(Update, start_game.run_if(on_message::<LoadCompleteMessage>));
    }
}

#[derive(States, Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    None,
    Start,
}

fn start_game(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::Srgba(Srgba::hex("9c6024").unwrap())));
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: FixedVertical {
                viewport_height: 1080.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.set_state(GameState::Start);
}
