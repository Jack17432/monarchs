mod editor;
mod gameplay;
mod third_party;
mod ui_camera;

use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::{log, log::LogPlugin};
use bevy_console::make_layer;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Monarchs descent".to_string(),
                        fit_canvas_to_parent: true,
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: log::Level::INFO,
                    filter: "wgpu=error,naga=warn,capture_bevy_logs=info".to_owned(),
                    custom_layer: make_layer,
                }),
        );

        app.insert_state(AppState::Gameplay)
            .add_sub_state::<GameState>();

        // Third party plugins
        app.add_plugins(third_party::plugin);

        // Development plugins
        app.add_plugins(editor::plugin);

        // Gameplay plugins
        app.add_plugins(gameplay::plugin)
            .add_plugins(ui_camera::plugin);
    }
}

#[derive(States, Hash, Eq, PartialEq, Clone, Copy, Debug)]
#[states(scoped_entities)]
enum AppState {
    Gameplay,
}

#[derive(SubStates, Hash, Eq, PartialEq, Clone, Copy, Debug, Default)]
#[source(AppState = AppState::Gameplay)]
#[states(scoped_entities)]
pub(crate) enum GameState {
    Playing,
    Inventory,
    #[default]
    Settings,
}

enum CameraOrder {
    World,
    ViewModel,
    Ui,
}

impl From<CameraOrder> for isize {
    fn from(order: CameraOrder) -> Self {
        order as isize
    }
}

pub(crate) const DEFAULT_RENDER_LAYER: usize = 0;
pub(crate) const VIEW_MODEL_RENDER_LAYER: usize = 1;
pub(crate) const UI_RENDER_LAYER: usize = 2;
