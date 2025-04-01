use bevy::prelude::*;
use parry3d::math::Point;
use parry3d::na::Vector;
use std::collections::HashMap;
use std::hash::Hash;

pub mod solver;

#[derive(Component, Debug, Copy, Clone, Reflect, Eq, PartialEq)]
#[require(LinerVelocity)]
pub enum PhysicsBodyType {
    Static,
    Dynamic,
}

#[derive(Component, Debug, Copy, Clone, Reflect, PartialEq, Default)]
pub struct LinerVelocity(pub Vec3);

impl LinerVelocity {
    pub const ZERO: Self = Self(Vec3::ZERO);
}

#[derive(Component, Debug, Copy, Clone, Reflect, PartialEq)]
pub struct LinerDamping(pub f64);

impl Default for LinerDamping {
    fn default() -> Self {
        Self(0.995)
    }
}

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

#[derive(Debug)]
pub struct CollisionInfo {
    entity_1: Entity,
    entity_2: Entity,

    dist: f32,
}

#[derive(Resource, Debug, Default)]
pub struct Collisions(HashMap<(Entity, Entity), CollisionInfo>);
