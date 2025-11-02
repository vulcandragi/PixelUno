mod plugins;

use crate::plugins::card::CardPlugin;
use crate::plugins::debug::DebugPlugin;
use crate::plugins::deck::DeckPlugin;
use crate::plugins::game::GamePlugin;
use crate::plugins::hand::HandPlugin;
use crate::plugins::load::LoadPlugin;
use crate::plugins::player::PlayerPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            LoadPlugin,
            GamePlugin,
            DebugPlugin,
            CardPlugin,
            DeckPlugin,
            PlayerPlugin,
            HandPlugin,
        ))
        .run();
}
