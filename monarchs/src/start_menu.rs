use crate::GameState;
use bevy::prelude::*;
use bevy_egui::*;

#[derive(Default)]
pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            run_start_menu.run_if(in_state(GameState::StartMenu)),
        );
    }
}

fn run_start_menu(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
