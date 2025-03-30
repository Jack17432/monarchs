use crate::core::{LinerVelocity, PhysicsBodyType};
use bevy::prelude::*;

const MAX_BOUNCES: usize = 5;
const SKIN_WIDTH: f64 = 0.015;

pub struct PhysicsSolverPlugin;

impl Plugin for PhysicsSolverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, integrate_velocities);
    }
}

fn integrate_velocities(mut q_bodies: Query<(&mut LinerVelocity, &PhysicsBodyType)>, time: Res<Time>) {
    let time_delta = time.delta_secs_f64();

    q_bodies.par_iter_mut().for_each(|(mut lin_vel, body_type)| {
        if *body_type == PhysicsBodyType::Static {
            return;
        }
    });
}
