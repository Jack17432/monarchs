pub mod vessel_switch;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, vessel_switch::vessel_switch_system);
    }
}
