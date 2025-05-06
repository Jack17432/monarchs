use bevy::prelude::*;
use bevy_console::{ConsoleConfiguration, ConsolePlugin};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ConsolePlugin);

    app.insert_resource(ConsoleConfiguration {
        ..Default::default()
    });
}
