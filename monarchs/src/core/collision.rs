use crate::core::Velocity;
use bevy::prelude::*;
use bevy::transform::systems::propagate_transforms;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Collisions>();

        app.add_systems(
            PostUpdate,
            (reset_collision_objs, sat_collider_detection)
                .chain()
                .before(propagate_transforms),
        );
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    mesh: Mesh,
    transformed_mesh: Mesh,
}

impl Collider {
    pub fn from_cuboid(mesh: Cuboid) -> Self {
        let mut mesh = mesh.mesh().build();
        mesh.compute_normals();
        Self {
            mesh: mesh.clone(),
            transformed_mesh: mesh,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct Collisions(pub Vec<(Entity, Entity)>);

pub fn reset_collision_objs(mut collisions: ResMut<Collisions>) {
    collisions.0.clear();
}

pub fn sat_collider_detection(
    mut q_objects: Query<(Entity, &mut Transform, &mut Collider, Option<&mut Velocity>)>,
    mut collisions: ResMut<Collisions>,
) {
    let collision_buffer = -0.01;

    q_objects
        .par_iter_mut()
        .for_each(|(_, transform, mut collider, _)| {
            collider.transformed_mesh = collider.mesh.clone().transformed_by(transform.clone());
            collider.transformed_mesh.compute_normals();
        });

    let mut iter = q_objects.iter_combinations_mut();
    while let Some(
        [
            (entity_1, mut transform_1, collider_1, opt_velocity_1),
            (entity_2, mut transform_2, collider_2, opt_velocity_2),
        ],
    ) = iter.fetch_next()
    {
        let mut collision_ab = f32::MIN;
        let mut normal_ab = Dir3::Y;
        for triangle in collider_1.transformed_mesh.triangles().unwrap() {
            let normal = triangle.normal().unwrap();

            let mut axis_length = f32::MAX;
            for triangle_other in collider_2.transformed_mesh.triangles().unwrap() {
                for vertex in triangle_other.vertices {
                    axis_length = axis_length.min(normal.dot(vertex - triangle.vertices[0]));
                }
            }
            if axis_length > collision_ab {
                collision_ab = axis_length;
                normal_ab = normal;
            }
        }

        let mut collision_ba = f32::MIN;
        let mut normal_ba = Dir3::Y;
        for triangle in collider_2.transformed_mesh.triangles().unwrap() {
            let normal = triangle.normal().unwrap();

            let mut axis_length = f32::MAX;
            for triangle_other in collider_1.transformed_mesh.triangles().unwrap() {
                for vertex in triangle_other.vertices {
                    axis_length = axis_length.min(normal.dot(vertex - triangle.vertices[0]));
                }
            }
            if axis_length > collision_ba {
                collision_ba = axis_length;
                normal_ba = normal;
            }
        }

        let mut furthest_intersection = 0.0;
        let mut furthest_normal = Dir3::Y;
        if collision_ab > collision_ba {
            furthest_intersection = collision_ab;
            furthest_normal = normal_ab;
        } else {
            furthest_intersection = collision_ba;
            furthest_normal = normal_ba;
        }

        if furthest_intersection < collision_buffer {
            // info!(
            //     furthest_intersection = ?furthest_intersection,
            //     furthest_normal = ?furthest_normal,
            //     collision_ab = ?collision_ab,
            //     collision_ba = ?collision_ba,
            //     entity_a = ?entity_1,
            //     entity_b = ?entity_2,
            //     transform_a = ?transform_1,
            //     transform_b = ?transform_2,
            //     "Collision detected"
            // );

            collisions.0.push((entity_1, entity_2));

            let mut shift = furthest_intersection;
            if opt_velocity_1.is_some() && opt_velocity_2.is_some() {
                shift /= 2.0;
            }

            if let Some(mut velocity) = opt_velocity_1 {
                transform_1.translation += furthest_normal * shift;
                let impulse = velocity.0 + (velocity.0 * *furthest_normal);
                velocity.0 = impulse;
            }
            if let Some(mut velocity) = opt_velocity_2 {
                transform_2.translation += furthest_normal * shift;
                let impulse = velocity.0 + (velocity.0 * *furthest_normal);
                velocity.0 = impulse;
            }
        }
    }
}
