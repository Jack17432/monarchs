use bevy::prelude::*;
use parry3d::math::Point;
use parry3d::na::Vector;

pub mod solver;

#[derive(Component, Debug, Copy, Clone, Reflect, Eq, PartialEq)]
#[require(LinerVelocity)]
pub enum PhysicsObject {
    Static,
    Dynamic,
}

#[derive(Component, Debug, Copy, Clone, Reflect, PartialEq, Default)]
pub struct LinerVelocity(pub Vec3);

#[derive(Component, Debug)]
pub struct Collider {
    pub collider: parry3d::shape::SharedShape,
}

impl Collider {
    pub fn from_cuboid(half_x: f32, half_y: f32, half_z: f32) -> Self {
        Self {
            collider: parry3d::shape::SharedShape::cuboid(half_x, half_y, half_z),
        }
    }

    pub fn from_capsule(a: Vec3, b: Vec3, radius: f32) -> Self {
        Self {
            collider: parry3d::shape::SharedShape::capsule(
                Point::from(a.to_array()),
                Point::from(b.to_array()),
                radius,
            ),
        }
    }
}
