pub mod souls;
pub mod vessels;

use crate::void_born::souls::{ChangeNextVesselEvent, VesselSwapEvent};
use bevy::prelude::*;

pub struct VoidBornPlugin;

impl Plugin for VoidBornPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VesselSwapEvent>()
            .add_event::<ChangeNextVesselEvent>()
            .add_systems(
                Update,
                (souls::vessel_swap_system, souls::vessel_change_next_system),
            );
    }
}
