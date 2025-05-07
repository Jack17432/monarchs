use crate::gameplay::input::{CloseInventory, InventoryActions, OpenInventory, PlayerActions};
use crate::GameState;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(open_inventory)
        .add_observer(close_inventory);
}

fn open_inventory(
    trigger: Trigger<Started<OpenInventory>>,
    mut commands: Commands,
    curr_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *curr_state != GameState::Playing {
        error!(
            ?curr_state,
            ?trigger,
            "Entering inventory from an incorrect state"
        );
        panic!("Entering inventory from an incorrect state");
    }

    debug!("opening inventory");

    commands
        .entity(trigger.target())
        .remove::<Actions<PlayerActions>>()
        .insert(Actions::<InventoryActions>::default());

    next_state.set(GameState::Inventory);
}

fn close_inventory(
    trigger: Trigger<Started<CloseInventory>>,
    mut commands: Commands,
    curr_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *curr_state != GameState::Inventory {
        error!(
            ?curr_state,
            ?trigger,
            "Closing inventory from an incorrect state"
        );
        panic!("Closing inventory from an incorrect state");
    }

    debug!("Closing inventory");

    commands
        .entity(trigger.target())
        .remove::<Actions<InventoryActions>>()
        .insert(Actions::<PlayerActions>::default());

    next_state.set(GameState::Playing);
}
