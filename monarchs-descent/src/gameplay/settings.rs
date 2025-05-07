use crate::gameplay::input::{CloseSettings, OpenSettings, PlayerActions, SettingsActions};
use crate::GameState;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(open_settings).add_observer(close_settings);
}

fn open_settings(
    trigger: Trigger<Fired<OpenSettings>>,
    mut commands: Commands,
    curr_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *curr_state != GameState::Playing {
        error!(
            ?curr_state,
            ?trigger,
            "Entering settings from an incorrect state"
        );
        panic!("Entering settings from an incorrect state");
    }

    debug!("opening settings");

    commands
        .entity(trigger.target())
        .remove::<Actions<PlayerActions>>()
        .insert(Actions::<SettingsActions>::default());

    next_state.set(GameState::Settings);
}

fn close_settings(
    trigger: Trigger<Fired<CloseSettings>>,
    mut commands: Commands,
    curr_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *curr_state != GameState::Settings {
        error!(
            ?curr_state,
            ?trigger,
            "Exiting settings from an incorrect state"
        );
        panic!("Exiting settings from an incorrect state");
    }

    info!("Closing settings");

    commands
        .entity(trigger.target())
        .remove::<Actions<SettingsActions>>()
        .insert(Actions::<PlayerActions>::default());

    next_state.set(GameState::Playing);
}
