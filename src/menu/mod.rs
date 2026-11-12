use bevy::app::Plugin;

use crate::menu::main::MainMenuPlugin;

mod main;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(MainMenuPlugin);
    }
}
