use crate::plugins::card::{AddCard, SpawnCard};
use crate::plugins::game::GameState;
use crate::plugins::hand::Hand;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), setup);
    }
}

#[derive(Component)]
pub struct Player {
    pub cards: i32,
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Player"),
            Player { cards: 0 },
            Transform::default(),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            children![(
                Hand {
                    cards: HashMap::new(),
                },
                Transform::from_xyz(0., -400., 0.),
                GlobalTransform::default(),
                InheritedVisibility::default(),
                children![]
            )],
        ))
        .observe(on_add_card);
}

fn on_add_card(
    add_card: On<AddCard>,
    mut players: Query<(&mut Player, Option<&Children>)>,
    hands: Query<&mut Hand>,
    mut message_writer: MessageWriter<SpawnCard>,
) {
    if let Ok((mut player, children)) = players.get_mut(add_card.entity) {
        player.cards += 1;

        if let Some(children) = children {
            for child in children.iter() {
                if hands.get(child).is_ok() {
                    let card = add_card.card.clone();
                    message_writer.write(SpawnCard {
                        entity: child,
                        card,
                    });
                }
            }
        }
    }
}
