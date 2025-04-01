use crate::core::{Collider, CollisionInfo, Collisions, LinerVelocity, PhysicsBodyType};
use bevy::prelude::*;
use parry3d::math::Isometry;
use parry3d::na::Vector3;

const SKIN_WIDTH: f32 = 0.015;
const GRAVITY: Vec3 = Vec3::new(0.0, 0.0, -10.0);

pub struct PhysicsSolverPlugin;

impl Plugin for PhysicsSolverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Collisions>();

        app.add_systems(
            FixedUpdate,
            (integrate_velocities, integrate_position, detect_collisions).chain(),
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
            PhysicsBodyType::Dynamic => {
                lin_vel.0 += GRAVITY * time_delta;
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
            PhysicsBodyType::Dynamic => {
                transform.translation += lin_vel.0 * time_delta;
            }
            PhysicsBodyType::Static => (),
        });
}

fn detect_collisions(
    q_bodies: Query<(Entity, &Transform, &Collider)>,
    mut collisions: ResMut<Collisions>,
) {
    collisions.0.clear();

    q_bodies.iter_combinations().for_each(
        |([
            (entity_1, transform_1, collider_1),
            (entity_2, transform_2, collider_2),
        ])| {
            let iso_1 = make_iso(
                transform_1.translation,
                transform_1.rotation.to_euler(EulerRot::XYZ).into(),
            );
            let iso_2 = make_iso(
                transform_2.translation,
                transform_2.rotation.to_euler(EulerRot::XYZ).into(),
            );

            let Ok(opt_contact) = parry3d::query::contact(
                &iso_1,
                collider_1.collider.0.as_ref(),
                &iso_2,
                collider_2.collider.0.as_ref(),
                -SKIN_WIDTH,
            ) else {
                return;
            };

            let Some(contact) = opt_contact else {
                return;
            };

            collisions.0.insert(
                (entity_1, entity_2),
                CollisionInfo {
                    entity_1,
                    entity_2,

                    dist: contact.dist + SKIN_WIDTH,
                },
            );
        },
    );
}

fn make_iso(transform: Vec3, rotate: Vec3) -> Isometry<f32> {
    Isometry::new(
        Vector3::new(transform.x, transform.y, transform.z),
        Vector3::new(rotate.x, rotate.y, rotate.z),
    )
}
