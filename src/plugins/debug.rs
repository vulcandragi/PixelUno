use bevy::app::Plugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    #[cfg(feature = "debug")]
    fn build(&self, app: &mut bevy::app::App) {
        use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

        app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::default()));
    }

    #[cfg(not(feature = "debug"))]
    fn build(&self, app: &mut bevy::app::App) {}
}
