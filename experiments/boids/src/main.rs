#![feature(map_try_insert)]

use std::collections::HashMap;

use bevy::color::palettes::basic::AQUA;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

const BOID_TRIG: Triangle2d = Triangle2d::new(Vec2::new(0.0, 15.0),
                                              Vec2::new(10.0, -15.0),
                                              Vec2::new(-10.0, -15.0));
const NUM_OF_BOIDS: usize = 100;
const BOID_REPULSE_RANGE: f32 = 50.0;
const BOIDS_RULE1_FACTOR: f32 = 100.0;
const BOIDS_RULE2_FACTOR: f32 = 5.0;
const BOIDS_RULE3_FACTOR: f32 = 100.0;
const BOID_VEL_LIMIT: f32 = 500.0;
const BOID_MOUSE_ATTRACTION_FACTOR: f32 = 50.0;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, spawn_boids).add_systems(Update, (boid_rule1, boid_rule2, boid_rule3, boid_rule_follow_mouse, limit_boid_vel).chain()).insert_resource(Time::<Fixed>::from_hz(1000.0)).add_systems(FixedUpdate, update_position).run();
}

#[derive(Component, Debug)]
struct Boid;

#[derive(Component, Debug)]
struct Velocity(Vec3);

#[derive(Component)]
struct MainCamera;

fn spawn_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_window: Single<&Window, With<PrimaryWindow>>,
) {
    commands.spawn((Camera2d, MainCamera));

    let mut rng = rand::rng();
    let half_width = (q_window.width() / 2.0) as i32;
    let half_height = (q_window.height() / 2.0) as i32;

    for _ in 0..NUM_OF_BOIDS {
        commands.spawn((
            Velocity(Vec3::new(0.0, 0.0, 0.0)),
            Transform::from_xyz(rng.random_range(-half_width..half_width) as f32,
                                rng.random_range(-half_height..half_height) as f32,
                                0.0),
            Mesh2d(meshes.add(BOID_TRIG)),
            MeshMaterial2d(materials.add(Color::from(AQUA))),
            Boid
        ));
    }
}

fn boid_rule1(
    mut query: Query<(Entity, &mut Velocity, &Transform), With<Boid>>,
) {
    let mut collector: HashMap<Entity, Vec3> = HashMap::new();
    let mut iter = query.iter_combinations_mut();

    while let Some([(entity1, _, transform1),
                   (entity2, _, transform2)]) = iter.fetch_next() {
        if !collector.contains_key(&entity1) {
            collector.try_insert(entity1.clone(), Vec3::ZERO).unwrap();
        }
        if !collector.contains_key(&entity2) {
            collector.try_insert(entity2.clone(), Vec3::ZERO).unwrap();
        }

        *collector.get_mut(&entity1).unwrap() += transform2.translation;
        *collector.get_mut(&entity2).unwrap() += transform1.translation;
    }

    for (entity, mut vel, transform) in query.iter_mut() {
        let transform_target = collector.get(&entity).unwrap();
        vel.0 += ((transform_target / NUM_OF_BOIDS as f32) - transform.translation) / BOIDS_RULE1_FACTOR;
    }
}

fn boid_rule2(
    mut query: Query<(Entity, &mut Velocity, &Transform), With<Boid>>,
) {
    let mut collector: HashMap<Entity, Vec3> = HashMap::new();

    let mut iter = query.iter_combinations_mut();
    while let Some([(entity1, _, transform1),
                   (entity2, _, transform2)]) = iter.fetch_next() {
        if !collector.contains_key(&entity1) {
            collector.try_insert(entity1.clone(), Vec3::ZERO).unwrap();
        }
        if !collector.contains_key(&entity2) {
            collector.try_insert(entity2.clone(), Vec3::ZERO).unwrap();
        }

        if (transform1.translation - transform2.translation).abs().length() < BOID_REPULSE_RANGE {
            *collector.get_mut(&entity1).unwrap() += transform1.translation - transform2.translation;
            *collector.get_mut(&entity2).unwrap() += transform2.translation - transform1.translation;
        }
    }

    for (entity, mut vel, _) in query.iter_mut() {
        let transform_target = collector.get(&entity).unwrap();
        vel.0 += transform_target / BOIDS_RULE2_FACTOR;
    }
}

fn boid_rule3(
    mut query: Query<(Entity, &mut Velocity, &Transform), With<Boid>>,
) {
    let mut collector: HashMap<Entity, Vec3> = HashMap::new();

    let mut iter = query.iter_combinations_mut();
    while let Some([(entity1, velocity1, _),
                   (entity2, velocity2, _)]) = iter.fetch_next() {
        if !collector.contains_key(&entity1) {
            collector.try_insert(entity1.clone(), Vec3::ZERO).unwrap();
        }
        if !collector.contains_key(&entity2) {
            collector.try_insert(entity2.clone(), Vec3::ZERO).unwrap();
        }

        *collector.get_mut(&entity1).unwrap() += velocity2.0;
        *collector.get_mut(&entity2).unwrap() += velocity1.0;
    }

    for (entity, mut vel, _) in query.iter_mut() {
        let transform_target = collector.get(&entity).unwrap();
        let curr_vel = vel.0.clone();
        vel.0 += ((transform_target / (NUM_OF_BOIDS - 1) as f32) - curr_vel) / BOIDS_RULE3_FACTOR;
    }
}

fn boid_rule_follow_mouse(
    mut query: Query<(&mut Velocity, &Transform), With<Boid>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position().and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok()).map(|ray| ray.origin.truncate())
    {
        for (mut velocity, transform) in &mut query {
            velocity.0 += (Vec3::from((world_position, 0.0)) - transform.translation) / BOID_MOUSE_ATTRACTION_FACTOR;
        }
    }
}

fn limit_boid_vel(
    mut query: Query<&mut Velocity, With<Boid>>,
) {
    for mut velocity in &mut query {
        if velocity.0.length() > BOID_VEL_LIMIT {
            velocity.0 = (velocity.0 / velocity.0.length()) * BOID_VEL_LIMIT;
        }
    }
}

fn update_position(
    mut query: Query<(&mut Transform, &Velocity), With<Boid>>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let half_width = q_window.width() / 2.0;
    let half_height = q_window.height() / 2.0;

    for (mut transform, velocity) in &mut query {
        // let mut transform: Transform = transform;
        transform.translation += velocity.0 * time.delta_secs();
        transform.rotation = Quat::from_rotation_z(-velocity.0.x.atan2(velocity.0.y));

        if transform.translation.x > half_width {
            transform.translation.x -= q_window.width();
        } else if transform.translation.x < -half_width {
            transform.translation.x += q_window.width();
        }

        if transform.translation.y > half_height {
            transform.translation.y -= q_window.height();
        } else if transform.translation.y < -half_height {
            transform.translation.y += q_window.height();
        }
    }
}