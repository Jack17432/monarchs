use crate::gameplay::input::{Inventory, OnFoot, OpenInventory};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(open_inventory);
}

fn open_inventory(trigger: Trigger<Started<OpenInventory>>, mut commands: Commands) {
    info!("Opening inventory");

    commands
        .entity(trigger.target())
        .insert(Actions::<Inventory>::default());
}
