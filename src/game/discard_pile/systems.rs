use bevy::{
    ecs::{
        entity::Entity,
        hierarchy::{ChildOf, Children},
        query::{Changed, With},
        system::{Query, Single},
    },
    math::{Quat, Vec3},
    transform::components::Transform,
};
use rand::random_range;

use crate::game::{
    card::components::{Card, CardVisibility},
    discard_pile::components::DiscardPile,
};

type RandomDirectionCardQuery<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static mut Transform, &'static mut CardVisibility),
    (With<Card>, Changed<ChildOf>),
>;

pub fn random_direction(
    discard_pile: Single<(&DiscardPile, &Children)>,
    mut card: RandomDirectionCardQuery,
) {
    let (discard_pile, children) = discard_pile.into_inner();

    let Some(entity) = discard_pile.last_card else {
        return;
    };

    let Ok((card_entity, mut transform, mut card_visibility)) = card.get_mut(entity) else {
        return;
    };

    if card_entity != entity {
        return;
    }

    transform.translation = Vec3 {
        x: random_range(-10..10) as f32,
        y: random_range(-10..10) as f32,
        z: children.iter().count() as f32 + 1.,
    };
    transform.rotation = Quat::from_rotation_z(random_range(-1.0..1.0));
    *card_visibility = CardVisibility::FaceUp;
}
