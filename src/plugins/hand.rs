use crate::plugins::card::Card;
use bevy::app::App;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

pub struct HandPlugin;

impl Plugin for HandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, position_cards);
    }
}

#[derive(Component)]
pub struct Hand {
    pub cards: HashMap<Entity, Card>,
}

fn position_cards(hands: Query<&Hand>, mut cards: Query<&mut Transform, With<Card>>) {
    for hand in hands {
        let total_cards = cards.count();

        for (i, (entity, _)) in hand.cards.iter().enumerate() {
            if let Ok(mut transform) = cards.get_mut(*entity) {
                transform.translation.x = (50 * i) as f32;
                transform.translation.z = (-(total_cards as f32)) + i as f32;
            }
        }
    }
}
