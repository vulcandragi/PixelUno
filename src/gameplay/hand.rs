use bevy::{
    ecs::{
        component::Component,
        event::EntityEvent,
        hierarchy::Children,
        observer::On,
        query::With,
        system::{Commands, Query},
    },
    math::Vec3,
    transform::components::Transform,
};

use crate::{
    events::Spawn,
    gameplay::card::{AddCard, Card},
};

#[derive(Component)]
pub struct Hand;

impl Hand {
    pub fn on_spawn(_: On<Spawn<Hand>>, mut commands: Commands) {
        commands
            .spawn((Hand, Transform::from_xyz(0., -200., 0.)))
            .observe(Self::on_cards_update);
    }

    fn on_cards_update(
        event: On<AddCard>,
        query_hand: Query<&Children, With<Hand>>,
        mut query_card: Query<&mut Transform, With<Card>>,
    ) {
        let Ok(children) = query_hand.get(event.event_target()) else {
            return;
        };

        let card_count = children.iter().count();
        let off_set = ((card_count as f32 * 60.) / 2.) + 30.;

        for (index, card_entity) in children.iter().enumerate() {
            let Ok(mut transform) = query_card.get_mut(*card_entity) else {
                continue;
            };

            transform.translation = Vec3 {
                x: (-off_set) + (60. * (index + 1) as f32),
                z: index as f32,
                ..Default::default()
            }
        }
    }
}
