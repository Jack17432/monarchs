use bevy::prelude::*;
use crate::core::LinerVelocity;

const MAX_BOUNCES: usize = 5;
const SKIN_WIDTH: f32 = 0.015;

pub struct PhysicsSolverPlugin;

impl Plugin for PhysicsSolverPlugin {
    fn build(&self, app: &mut App) {

    }
}


/// https://www.youtube.com/watch?v=YR6Q7dUz2uk
fn collide_and_slide(position: Transform, velocity: LinerVelocity, depth: usize) -> LinerVelocity {
    if (depth >= MAX_BOUNCES) {
        return LinerVelocity(Vec3::ZERO);
    }

    let dist = velocity.0.length() + SKIN_WIDTH;



    velocity
}