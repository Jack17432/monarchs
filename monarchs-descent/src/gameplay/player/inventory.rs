use crate::gameplay::input::{CloseInventory, InventoryActions, OpenInventory, PlayerActions};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(open_inventory)
        .add_observer(close_inventory);
}

fn open_inventory(trigger: Trigger<Started<OpenInventory>>, mut commands: Commands) {
    info!("Opening inventory");

    commands
        .entity(trigger.target())
        .remove::<Actions<PlayerActions>>()
        .insert(Actions::<InventoryActions>::default());
}

fn close_inventory(trigger: Trigger<Started<CloseInventory>>, mut commands: Commands) {
    info!("Closing inventory");

    commands
        .entity(trigger.target())
        .remove::<Actions<InventoryActions>>()
        .insert(Actions::<PlayerActions>::default());
}
