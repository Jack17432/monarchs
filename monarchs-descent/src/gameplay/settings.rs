use crate::gameplay::input::{CloseSettings, OpenSettings, PlayerActions, SettingsActions};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(open_settings).add_observer(close_settings);
}

fn open_settings(trigger: Trigger<Fired<OpenSettings>>, mut commands: Commands) {
    info!("opening settings");

    commands
        .entity(trigger.target())
        .remove::<Actions<PlayerActions>>()
        .insert(Actions::<SettingsActions>::default());
}

fn close_settings(trigger: Trigger<Fired<CloseSettings>>, mut commands: Commands) {
    info!("opening settings");

    commands
        .entity(trigger.target())
        .remove::<Actions<SettingsActions>>()
        .insert(Actions::<PlayerActions>::default());
}
