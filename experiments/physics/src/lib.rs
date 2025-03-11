use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum PhysicsState {
    #[default]
    Paused,
    Running,
}

pub struct VerletIntegrationPlugin;

#[derive(Component, Debug)]
#[require(Transform, Velocity)]
pub struct PhysicsObject;

#[derive(Component, Debug)]
pub struct Collider {
    radius: f32,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[derive(Component, Debug, Default)]
pub struct Velocity(Vec3);

impl Plugin for VerletIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PhysicsState>()
            .insert_resource(Time::<Fixed>::from_hz(500.0))
            .add_systems(
                FixedUpdate,
                (collision, step)
                    .chain()
                    .run_if(in_state(PhysicsState::Running)),
            );
    }
}

impl Velocity {
    pub fn new(vec: Vec3) -> Self {
        Self(vec)
    }
}

fn step(
    mut objects: Query<(&mut Transform, &mut Velocity), With<PhysicsObject>>,
    time: Res<Time<Fixed>>,
) {
    objects
        .par_iter_mut()
        .for_each(|(mut transform, mut velocity)| {
            transform.translation += velocity.0 * time.delta_secs();
            velocity.0 += Vec3::new(0.0, -9.8, 0.0) * time.delta_secs();
        });
}

fn collision(mut objects: Query<(&mut Transform, &mut Velocity, &Collider), With<PhysicsObject>>) {
    let mut iter = objects.iter_combinations_mut();
    while let Some(
        [
            (mut transform1, mut velocity1, collider1),
            (mut transform2, mut velocity2, collider2),
        ],
    ) = iter.fetch_next()
    {
        let dist = transform1.translation.distance(transform2.translation);
        if dist < collider1.radius + collider2.radius {
            let shift = (dist - (collider1.radius + collider2.radius)) / 2.0;
            let ab_dir = (transform2.translation - transform1.translation).normalize();
            let ba_dir = (transform1.translation - transform2.translation).normalize();

            (transform1.translation, transform2.translation) = (
                transform1.translation + (ab_dir * shift),
                transform2.translation + (ba_dir * shift),
            );

            velocity1.0 = velocity1.0 - (2.0 * velocity1.0.dot(ab_dir) * ab_dir);
            velocity2.0 = velocity2.0 - (2.0 * velocity2.0.dot(ba_dir) * ba_dir);
        }
    }
}
