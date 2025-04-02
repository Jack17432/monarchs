use bevy::prelude::*;
use parry3d::math::{Isometry, Point};
use parry3d::na::Vector3;
use std::collections::HashMap;

pub mod solver;

#[derive(Component, Debug, Copy, Clone, Reflect, Eq, PartialEq)]
#[require(LinerVelocity)]
pub enum PhysicsBodyType {
    Static,
    Controlled,
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
    pub manifolds: Vec<CollisionManifold>,
}

impl CollisionInfo {
    pub fn new(manifolds: Vec<CollisionManifold>) -> Self {
        Self { manifolds }
    }
}

#[derive(Debug)]
pub struct CollisionManifold {
    pub points: Vec<CollisionPoints>,
    pub normal: Vec3,
}

impl CollisionManifold {
    pub fn new(points: Vec<CollisionPoints>, normal: Vec3) -> Self {
        Self { points, normal }
    }
}

#[derive(Debug)]
pub struct CollisionPoints {
    pub penetration: f32,
}

impl CollisionPoints {
    pub fn new(penetration: f32) -> Self {
        Self { penetration }
    }
}

#[derive(Resource, Debug, Default)]
pub struct Collisions(HashMap<(Entity, Entity), CollisionInfo>);

fn make_iso(transform: Vec3, rotate: Vec3) -> Isometry<f32> {
    Isometry::new(
        Vector3::new(transform.x, transform.y, transform.z),
        Vector3::new(rotate.x, rotate.y, rotate.z),
    )
}
