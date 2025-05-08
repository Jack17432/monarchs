pub(super) mod inventory;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Item>();

    app.add_plugins(inventory::plugin);
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Item;
