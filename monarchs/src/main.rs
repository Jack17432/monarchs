use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use monarchs::GameState;
use monarchs::start_menu::StartMenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_state::<GameState>()
        .add_plugins(StartMenuPlugin)
        .run();
}
