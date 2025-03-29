use bevy::prelude::*;

#[derive(Bundle, Debug, Default)]
pub struct VesselBundle {
    vessel: Vessel,
}

#[derive(Component, Debug, Default)]
pub struct Vessel;
