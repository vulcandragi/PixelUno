use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::EntityEvent,
        hierarchy::Children,
        name::Name,
        observer::On,
        query::With,
        system::{Commands, Query, Single},
    },
    math::Vec3,
    picking::events::{Click, Pointer},
    transform::components::Transform,
};

use crate::{
    events::Spawn,
    gameplay::{
        card::{AddCard, Card, CardSpawnData},
        stack::Stack,
    },
};

#[derive(Component)]
pub struct Hand;

#[derive(EntityEvent)]
pub struct ReorderHand(Entity);

impl Hand {
    pub fn on_spawn(_: On<Spawn<Hand>>, mut commands: Commands) {
        commands
            .spawn((Hand, Transform::from_xyz(0., -200., 0.)))
            .insert(Name("Hand".into()))
            .observe(Self::on_reorder_hand)
            .observe(Self::on_add_card)
            .observe(Self::on_card_click);
    }

    fn on_reorder_hand(
        event: On<ReorderHand>,
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

    fn on_add_card(event: On<AddCard>, mut commands: Commands) {
        commands.trigger(ReorderHand(event.event_target()));
    }

    fn on_card_click(
        event: On<Pointer<Click>>,
        mut commands: Commands,
        query: Query<(Entity, &Card)>,
        stack: Single<(Entity, &Stack)>,
    ) {
        let Ok((entity, card)) = query.get(event.original_event_target()) else {
            return;
        };

        let Some(current_card) = &stack.1.last_card else {
            return;
        };

        if !current_card.check_card(card) {
            return;
        }

        commands.trigger(
            Spawn::<Card, CardSpawnData>::default()
                .with_data(CardSpawnData {
                    color: card.color,
                    symbol: card.symbol,
                })
                .with_parent(stack.0),
        );
        commands.entity(entity).despawn();
        commands.trigger(ReorderHand(event.event_target()));
    }
}
