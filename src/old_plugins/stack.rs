use crate::old_plugins::card::{AddCard, Card, SpawnCard};
use crate::old_plugins::game::GameState;
use bevy::prelude::*;

pub struct StackPlugin;
impl Plugin for StackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), setup);
    }
}

#[derive(Component)]
pub struct Stack {
    pub last_card: Option<Card>,
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Stack { last_card: None },
            Transform::default(),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            children![],
        ))
        .observe(on_add_card);
}

fn on_add_card(
    add_card: On<AddCard>,
    mut stack: Query<&mut Stack>,
    mut message_writer: MessageWriter<SpawnCard>,
) {
    if let Ok(mut stack) = stack.get_mut(add_card.entity) {
        stack.last_card = Some(add_card.card.clone());
        message_writer.write(SpawnCard {
            entity: add_card.entity,
            card: add_card.card.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
        });
    }
}
