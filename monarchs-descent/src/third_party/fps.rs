use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FramepacePlugin);

    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            enabled: true,
            ..default()
        },
    });
}
