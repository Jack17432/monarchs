pub mod collision;

use crate::core::collision::{sat_collider_detection, Collisions};
use bevy::math::bounding::IntersectsVolume;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (collision_move, calculate_gravity, integrator)
                .chain()
                .after(sat_collider_detection),
        );
    }
}

fn integrator(
    mut q_objects: Query<(
        &mut Transform,
        &mut Velocity,
        &Acceleration,
        &Mass,
        &Damping,
        &Forces,
    )>,
    time: Res<Time>,
) {
    let time_delta = time.delta_secs();

    q_objects.par_iter_mut().for_each(
        |(mut transform, mut velocity, acceleration, mass, damping, forces)| {
            transform.translation +=
                velocity.0 * time_delta + acceleration.0 * time_delta.powi(2) * 0.5;

            let mut res_accel = acceleration.0;
            res_accel += forces.0 * mass.get_inv_mass();

            velocity.0 = velocity.0 * damping.0.powf(time_delta) + acceleration.0 * time_delta;
        },
    )
}

fn calculate_gravity(mut q_objects: Query<(&mut Acceleration)>) {
    q_objects
        .par_iter_mut()
        .for_each(|(mut acceleration)| acceleration.0 = Vec3::new(0.0, 0.0, -1.0));
}

fn collision_move(
    collisions: Res<Collisions>,
    mut q_objects: Query<(&mut Transform, Option<&mut Velocity>)>,
) {
    for (entity_1, entity_2, distance, normal) in &collisions.0 {
        let Ok(
            [
                (mut transform1, mut velocity1),
                (mut transform2, mut velocity2),
            ],
        ) = q_objects.get_many_mut([*entity_1, *entity_2])
        else {
            continue;
        };

        let mut shift = *distance / 2.0;
        if velocity1.is_some() && velocity2.is_some() {
            shift /= 2.0;
        }

        if let Some(mut velocity) = velocity1 {
            transform1.translation = transform1.translation + (-normal.as_vec3() * (shift));
            velocity.0 = Vec3::ZERO;
            // velocity.0 = velocity.0 - (2.0 * velocity.0.dot(-normal.as_vec3()) * -normal.as_vec3()) * 0.1;
        }
        if let Some(mut velocity) = velocity2 {
            transform2.translation = transform2.translation + (normal.as_vec3() * (shift));
            velocity.0 = velocity.0 - (2.0 * velocity.0.dot(normal.as_vec3()) * normal.as_vec3()) * 0.1;
        }
    }
}

/// Unit is in **Kg**
#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Mass {
    mass: f32,
    inv_mass: f32,
}

impl Mass {
    /// Creates a new mass component in **Kg**
    pub fn new(mass: f32) -> Self {
        Self {
            mass,
            inv_mass: 1.0 / mass,
        }
    }

    /// Gets mass value in **Kg**
    fn get_mass(&self) -> f32 {
        self.mass
    }

    /// Gets 1/mass value in **Kg**
    fn get_inv_mass(&self) -> f32 {
        self.inv_mass
    }

    /// Sets the mass and the unit should be in **Kg**
    fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
        self.inv_mass = 1.0 / self.mass;
    }
}

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Damping(pub f32);

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Forces(pub Vec3);

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub transform: Transform,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub mass: Mass,
    pub damping: Damping,
    pub forces: Forces,
}
