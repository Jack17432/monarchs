use crate::controllers::player::Player;
use crate::void_born::souls::{BoundToVessel, NextVessel, OwnedVessels};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn vessel_switch_system(
    mut contexts: EguiContexts,
    soul: Single<(&BoundToVessel, &NextVessel, &OwnedVessels), With<Player>>,
) {
    egui::Window::new("Vessels").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Current vessel {:?}", soul.0.0));
        ui.label(format!("Next vessel {:?}", soul.1.0));
    });
}
