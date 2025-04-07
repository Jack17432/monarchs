use bevy::math::Vec3;
use bevy::prelude::{Component, Entity, Reflect, Resource};
use std::cmp::max;
use std::collections::HashMap;
use std::ops::Add;

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
    a: Vec3,
    b: Vec3,
}

impl Add<Vec3> for Collider {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        let a = self.a + rhs;
        let b = self.b + rhs;
        Self { a, b }
    }
}

#[derive(Debug)]
pub enum CollisionState {
    NoCollision,
    /// contains the distance to collision and the normal of the collision.
    Collision(f32, Vec3),
}

impl Collider {
    pub fn new(half_x: f32, half_y: f32, half_z: f32) -> Self {
        Self {
            a: Vec3::new(-half_x, -half_y, -half_z),
            b: Vec3::new(half_x, half_y, half_z),
        }
    }

    pub fn aabb_collision(&self, other: &Self) -> bool {
        let x = self.b.x.min(other.b.x) - self.a.x.max(other.a.x);
        let y = self.b.y.min(other.b.y) - self.a.y.max(other.a.y);
        let z = self.b.z.min(other.b.z) - self.a.z.max(other.a.z);

        (x > 0.0) && (y > 0.0) && (z > 0.0)
    }

    pub fn swept_aabb_collision(&self, vel: &LinerVelocity, other: &Self) -> CollisionState {
        fn calc_time(dist: f32, vel: f32) -> f32 {
            if vel == 0.0 {
                return if dist > 0.0 { f32::MIN } else { f32::MAX };
            }

            dist / vel
        }

        let x_entry: f32;
        let x_exit: f32;
        let y_entry: f32;
        let y_exit: f32;
        let z_entry: f32;
        let z_exit: f32;

        if vel.0.x > 0.0 {
            x_entry = calc_time(other.a.x - self.b.x, vel.0.x);
            x_exit = calc_time(other.b.x - self.a.x, vel.0.x);
        } else {
            x_entry = calc_time(other.b.x - self.a.x, vel.0.x);
            x_exit = calc_time(other.a.x - self.b.x, vel.0.x);
        }
        if vel.0.y > 0.0 {
            y_entry = calc_time(other.a.y - self.b.y, vel.0.y);
            y_exit = calc_time(other.b.y - self.a.y, vel.0.y);
        } else {
            y_entry = calc_time(other.b.y - self.a.y, vel.0.y);
            y_exit = calc_time(other.a.y - self.b.y, vel.0.y);
        }
        if vel.0.z > 0.0 {
            z_entry = calc_time(other.a.z - self.b.z, vel.0.z);
            z_exit = calc_time(other.b.z - self.a.z, vel.0.z);
        } else {
            z_entry = calc_time(other.b.z - self.a.z, vel.0.z);
            z_exit = calc_time(other.a.z - self.b.z, vel.0.z);
        }

        if x_entry < 0.0 && y_entry < 0.0 && z_entry < 0.0 {
            return CollisionState::NoCollision;
        }
        if x_entry > 1.0 && y_entry > 1.0 && z_entry > 1.0 {
            return CollisionState::NoCollision;
        }

        let entry = x_entry.max(y_entry).max(z_entry);
        let exit = x_exit.max(y_exit).max(z_exit);

        if entry > exit {
            return CollisionState::NoCollision;
        }

        let norm_x = if entry == x_entry {
            if vel.0.x > 0.0 { -1.0 } else { 1.0 }
        } else {
            0.0
        };
        let norm_y = if entry == y_entry {
            if vel.0.y > 0.0 { -1.0 } else { 1.0 }
        } else {
            0.0
        };
        let norm_z = if entry == z_entry {
            if vel.0.z > 0.0 { -1.0 } else { 1.0 }
        } else {
            0.0
        };

        let normal = Vec3::new(norm_x, norm_y, norm_z);

        CollisionState::Collision((vel.0 * entry).length(), normal)
    }
}
