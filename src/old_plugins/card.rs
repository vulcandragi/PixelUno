use crate::old_plugins::hand::Hand;
use crate::old_plugins::load::LoadState;
use crate::old_plugins::stack::Stack;
use bevy::prelude::*;
use bevy::window::{CursorIcon, SystemCursorIcon};
use bevy_asset_loader::prelude::*;
use num_enum::{FromPrimitive, IntoPrimitive};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(LoadState::Loading).load_collection::<CardAssets>(),
        )
        .add_systems(
            Update,
            (spawn_hand, spawn_stack).run_if(on_message::<SpawnCard>),
        )
        .add_message::<SpawnCard>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct CardAssets {
    #[asset(texture_atlas_layout(tile_size_x = 84, tile_size_y = 120, columns = 15, rows = 5))]
    pub cards: Handle<TextureAtlasLayout>,
    #[asset(path = "images/cards.png")]
    pub cards_texture: Handle<Image>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct Card {
    pub color: CardColor,
    pub symbol: CardSymbol,
}

impl Card {
    pub fn as_atlas_index(&self) -> usize {
        if self.color == CardColor::Black {
            return match self.symbol {
                CardSymbol::Color => 60usize,
                CardSymbol::Plus4 => 61usize,
                _ => 62usize,
            };
        }

        if self.symbol == CardSymbol::None || self.color == CardColor::None {
            63usize
        } else {
            ((u8::from(self.color)) * 15 + u8::from(self.symbol)) as usize
        }
    }
}

#[derive(IntoPrimitive, FromPrimitive, Reflect, Copy, Clone, Debug, Default, PartialEq)]
#[repr(u8)]
pub enum CardColor {
    Blue = 0,
    Yellow,
    Red,
    Green,
    Black,
    #[default]
    None,
}

#[derive(IntoPrimitive, FromPrimitive, Reflect, Copy, Clone, Debug, Default, PartialEq)]
#[repr(u8)]
pub enum CardSymbol {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Block,
    Reverse,
    Plus2,
    Color,
    Plus4,
    #[default]
    None,
}

#[derive(Component)]
pub struct Index(usize);

#[derive(EntityEvent)]
pub struct AddCard {
    pub entity: Entity,
    pub card: Card,
}

#[derive(Message)]
pub struct SpawnCard {
    pub entity: Entity,
    pub card: Card,
    pub transform: Transform,
}

fn create_card(spawn_card: &SpawnCard, assets: &Res<CardAssets>) -> impl Bundle {
    (
        spawn_card.card.clone(),
        Sprite {
            image: assets.cards_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.cards.clone(),
                index: spawn_card.card.as_atlas_index(),
            }),
            ..default()
        },
        spawn_card.transform,
    )
}

fn spawn_hand(
    mut commands: Commands,
    mut message_reader: MessageReader<SpawnCard>,
    assets: Res<CardAssets>,
    mut query: Query<&mut Hand>,
) {
    for spawn in message_reader.read() {
        if let Ok(mut hand) = query.get_mut(spawn.entity) {
            let card_entity = commands
                .spawn((create_card(spawn, &assets), Index(0), Pickable::default()))
                .observe(on_over)
                .observe(on_out)
                .id();

            commands.entity(spawn.entity).add_child(card_entity);
        }
    }
}

fn spawn_stack(
    mut commands: Commands,
    mut message_reader: MessageReader<SpawnCard>,
    assets: Res<CardAssets>,
) {
    for spawn in message_reader.read() {
        let card_entity = commands.spawn((create_card(spawn, &assets),)).id();

        commands.entity(spawn.entity).add_child(card_entity);
    }
}

fn on_over(
    hover: On<Pointer<Over>>,
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Card>>,
    window: Single<Entity, With<Window>>,
) {
    if let Ok(mut transform) = query.get_mut(hover.entity) {
        transform.translation.y += 25.;
        commands
            .entity(*window)
            .insert(CursorIcon::from(SystemCursorIcon::Pointer));
    }
}

fn on_out(
    hover: On<Pointer<Out>>,
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Card>>,
    window: Single<Entity, With<Window>>,
) {
    if let Ok(mut transform) = query.get_mut(hover.entity) {
        transform.translation.y += -25.;
        commands
            .entity(*window)
            .insert(CursorIcon::from(SystemCursorIcon::Default));
    }
}
