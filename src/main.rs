use bevy::{DefaultPlugins, app::App};

mod old_plugins;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
