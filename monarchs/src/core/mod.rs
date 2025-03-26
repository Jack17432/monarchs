use bevy::ecs::batching::BatchingStrategy;
use bevy::prelude::*;

#[derive(Component, Debug)]
#[require(Transform, Velocity, Acceleration)]
pub struct PhysicsObject;

#[derive(Default, Component, Debug, Clone, PartialEq)]
pub struct Velocity(pub Vec3);

#[derive(Default, Component, Debug, Clone, PartialEq)]
pub struct Acceleration(pub Vec3);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (friction_step, velocity_step).chain());
    }
}

fn velocity_step(
    mut q_objects: Query<(Entity, &mut Transform, &mut Velocity, &mut Acceleration), With<PhysicsObject>>,
    time: Res<Time>,
) {
    q_objects
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::default())
        .for_each(|(entity, mut transform, mut velocity, mut acceleration)| {
            info!(entity=?entity, transform=?transform, velocity=?velocity);
            transform.translation += velocity.0 * time.delta_secs();
            velocity.0 += acceleration.0 * time.delta_secs();
            acceleration.0 = Vec3::ZERO;
        });
}

fn friction_step(
    mut q_objects: Query<(&mut Velocity, &mut Acceleration), With<PhysicsObject>>,
    time: Res<Time>,
) {
    q_objects
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::default())
        .for_each(|(mut velocity, mut acceleration)| {
            let friction = velocity.0 * -0.1;
            velocity.0 += friction * time.delta_secs();
        });
}
