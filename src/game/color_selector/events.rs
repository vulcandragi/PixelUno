use bevy::{
    ecs::{
        entity::Entity,
        observer::On,
        system::{Commands, Query, Single},
    },
    log::info,
    picking::events::PointerClick,
};

use crate::game::{
    card::components::Card,
    color_selector::components::{ColorButton, ColorSelector},
    game_manager::events::ActionIntent,
};

pub fn on_select_color(
    on: On<PointerClick>,
    mut commands: Commands,
    color_selector: Single<(Entity, &ColorSelector)>,
    color_button: Query<&ColorButton>,
    mut cards: Query<&mut Card>,
) {
    info!("{on:?}");

    let (color_selector_entiry, ColorSelector(card_entity)) = color_selector.into_inner();

    let Ok(ColorButton(color)) = color_button.get(on.original_event_target()) else {
        return;
    };

    let Ok(mut card) = cards.get_mut(*card_entity) else {
        return;
    };

    card.color = *color;

    commands.trigger(ActionIntent::Card {
        next_card: *card_entity,
    });
    commands.entity(color_selector_entiry).despawn();
}
