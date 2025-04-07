use crate::core::physics::{LinerVelocity, PhysicsBodyType};
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

const GRAVITY: Vec3 = Vec3::new(0.0, 0.0, -10.0);

#[derive(Default, Debug)]
pub struct PhysicsSolverPlugin;

impl Plugin for PhysicsSolverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (integrate_velocities, integrate_position).chain(),
        );
    }
}

fn integrate_velocities(
    mut q_bodies: Query<(&mut LinerVelocity, &PhysicsBodyType)>,
    time: Res<Time>,
) {
    let time_delta = time.delta_secs();

    q_bodies
        .par_iter_mut()
        .for_each(|(mut lin_vel, body_type)| match body_type {
            PhysicsBodyType::Controlled => {
                // lin_vel.0 += GRAVITY * time_delta;
                lin_vel.0 *= Vec3::new(0.95, 0.95, 0.99);
            }
            PhysicsBodyType::Static => {
                *lin_vel = LinerVelocity::ZERO;
            }
        });
}

fn integrate_position(
    mut q_bodies: Query<(&mut Transform, &LinerVelocity, &PhysicsBodyType)>,
    time: Res<Time>,
) {
    let time_delta = time.delta_secs();

    q_bodies
        .par_iter_mut()
        .for_each(|(mut transform, lin_vel, body_type)| match body_type {
            PhysicsBodyType::Controlled => {
                transform.translation += lin_vel.0 * time_delta;
            }
            PhysicsBodyType::Static => (),
        });
}
