use crate::core::{
    Collider, CollisionInfo, CollisionManifold, CollisionPoints, Collisions, LinerVelocity,
    PhysicsBodyType,
};
use bevy::prelude::*;
use parry3d::na::RealField;
use parry3d::query::{ContactManifold, PersistentQueryDispatcher};

const SKIN_WIDTH: f32 = 0.015;
const GRAVITY: Vec3 = Vec3::new(0.0, 0.0, -10.0);

pub struct PhysicsSolverPlugin;

impl Plugin for PhysicsSolverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Collisions>();

        app.add_systems(
            FixedUpdate,
            (
                detect_collisions,
                controlled_collision_controller,
                integrate_velocities,
                integrate_position,
            )
                .chain(),
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
            PhysicsBodyType::Controlled => {
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
            let iso_1 = crate::core::make_iso(
                transform_1.translation,
                transform_1.rotation.to_euler(EulerRot::XYZ).into(),
            );
            let iso_2 = crate::core::make_iso(
                transform_2.translation,
                transform_2.rotation.to_euler(EulerRot::XYZ).into(),
            );

            // let Ok(opt_contact) = parry3d::query::contact(
            //     &iso_1,
            //     collider_1.collider.0.as_ref(),
            //     &iso_2,
            //     collider_2.collider.0.as_ref(),
            //     0.0,
            // ) else {
            //     return;
            // };
            //
            // let Some(contact) = opt_contact else {
            //     return;
            // };
            //
            // let normal = Vec3::new(contact.normal1.x, contact.normal1.y, contact.normal1.z);
            //
            // collisions.0.insert(
            //     (entity_1, entity_2),
            //     CollisionInfo {
            //         normal,
            //         penetration: -contact.dist,
            //     },
            // );

            let mut manifolds: Vec<ContactManifold<(), ()>> = Vec::new();

            let Ok(()) = parry3d::query::DefaultQueryDispatcher.contact_manifolds(
                &iso_1.inv_mul(&iso_2),
                collider_1.collider.0.as_ref(),
                collider_2.collider.0.as_ref(),
                0.0,
                &mut manifolds,
                &mut None,
            ) else {
                return;
            };

            if manifolds.len() <= 1
                && manifolds
                    .get(0)
                    .is_some_and(|contact| contact.points.len() == 0)
            {
                return;
            }

            let mut contact_info = Vec::new();
            for manifold in manifolds.iter_mut() {
                let mut contact_points = Vec::new();

                for contact_point_pair in &manifold.points {
                    contact_points.push(CollisionPoints::new(-contact_point_pair.dist))
                }

                contact_info.push(CollisionManifold::new(
                    contact_points,
                    Vec3::new(
                        manifold.local_n1.x,
                        manifold.local_n1.y,
                        manifold.local_n1.z,
                    ),
                ));
            }

            collisions
                .0
                .insert((entity_1, entity_2), CollisionInfo::new(contact_info));
        },
    );
}

fn controlled_collision_controller(
    collisions: Res<Collisions>,
    q_bodies: Query<&PhysicsBodyType>,
    mut q_body_controller: Query<(&mut Transform, &mut LinerVelocity, &PhysicsBodyType)>,
) {
    for (((entity_1, entity_2), collision_info)) in collisions.0.iter() {
        let is_first: bool;

        let (mut transform, mut lin_vel, _) = match q_bodies.get_many([*entity_1, *entity_2]) {
            Ok([PhysicsBodyType::Controlled, PhysicsBodyType::Controlled]) => continue,
            Ok([PhysicsBodyType::Controlled, _]) => {
                is_first = true;
                q_body_controller.get_mut(*entity_1).unwrap()
            }
            Ok([_, PhysicsBodyType::Controlled]) => {
                is_first = false;
                q_body_controller.get_mut(*entity_2).unwrap()
            }
            _ => continue,
        };

        for manifold in &collision_info.manifolds {
            let normal = if is_first {
                -manifold.normal
            } else {
                manifold.normal
            };

            let mut max_pen = f32::MIN;
            for contact in &manifold.points {
                if contact.penetration > 0.0 {
                    transform.translation += normal * contact.penetration;
                }
                max_pen = max_pen.max(contact.penetration);
            }

            if lin_vel.0.dot(normal) > 0.0 {
                continue;
            }

            let impulse = lin_vel.0.reject_from_normalized(normal);
            lin_vel.0 = impulse;
        }
    }
}
