use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct PlayerCamera;

pub enum CameraOrder {
    World,
    Ui,
}

impl From<CameraOrder> for isize {
    fn from(order: CameraOrder) -> Self {
        order as isize
    }
}
