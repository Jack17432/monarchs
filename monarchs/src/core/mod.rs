use bevy::math::bounding::{Aabb3d, IntersectsVolume};
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (calculate_gravity, integrator).chain());
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

// fn collision_detection(mut q_colliders: Query<(&Aabb3d, &mut Transform)>) {
//     let mut iter = q_colliders.iter_combinations_mut();
//     for ([(aabb_1, transform_1), (aabb_2, transform_2)]) in iter.fetch_next() {
//         if aabb_1.intersects(aabb_2) {
//             aabb_2
//         }
//     }
// }

fn calculate_gravity(mut q_objects: Query<(&mut Acceleration)>) {
    q_objects
        .par_iter_mut()
        .for_each(|(mut acceleration)| acceleration.0 = Vec3::new(0.0, 0.0, -10.0));
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
