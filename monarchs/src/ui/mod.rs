pub mod vessel;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, vessel::vessel_switch_system);
    }
}
