use bevy::math::bounding::Aabb3d;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum State {
    #[default]
    Paused,
    Running,
}

pub struct VerletIntegrationPlugin;

#[derive(Component, Debug)]
#[require(Transform, Velocity)]
pub struct VerletObject;

#[derive(Component, Debug)]
pub struct Collider;

#[derive(Component, Debug, Default)]
pub struct Velocity(Vec3);

impl Plugin for VerletIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<State>()
            .insert_resource(Time::<Fixed>::from_hz(500.0))
            .add_systems(FixedUpdate, step.run_if(in_state(State::Running)));
    }
}

impl Velocity {
    pub fn new(vec: Vec3) -> Self {
        Self(vec)
    }
}

fn step(
    mut objects: Query<(&mut Transform, &mut Velocity), With<VerletObject>>,
    time: Res<Time<Fixed>>,
) {
    objects
        .par_iter_mut()
        .for_each(|(mut transform, mut velocity)| {
            transform.translation += velocity.0 * time.delta_secs();
            velocity.0 += Vec3::new(0.0, -9.8, 0.0) * time.delta_secs();
        });
}