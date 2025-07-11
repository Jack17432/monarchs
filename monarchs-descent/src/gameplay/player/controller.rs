use crate::gameplay::input::{Crouch, Jump, Move, PlayerActions};
use crate::gameplay::player::Player;
use crate::gameplay::player::inventory::Holding;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_inspector_egui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Stance>()
        .register_type::<MovementDamping>()
        .register_type::<MovementAcceleration>()
        .register_type::<CrouchModifier>()
        .register_type::<JumpImpulse>();

    app.add_observer(apply_player_movement)
        .add_observer(apply_player_jump);

    app.add_systems(
        Update,
        (
            check_grounded,
            (apply_player_stance, apply_player_movement_damping),
        )
            .chain(),
    );
}

#[derive(Bundle)]
pub(super) struct PlayerControllerBundle {
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    stance: Stance,
    movement_speed: MovementAcceleration,
    movement_damping: MovementDamping,
    jump_impulse: JumpImpulse,
    crouch_modifier: CrouchModifier,
}

impl PlayerControllerBundle {
    pub fn new(collider: Collider) -> Self {
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vec3::ONE * 0.99, 10);

        Self {
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vec3::ZERO, Quat::default(), Dir3::NEG_Y)
                .with_max_distance(0.2),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            stance: Stance::default(),
            movement_speed: MovementAcceleration::new(5.0, 0.2),
            movement_damping: MovementDamping::new(0.3, 0.98),
            jump_impulse: JumpImpulse(5.0),
            crouch_modifier: CrouchModifier(0.3),
        }
    }
}

#[derive(Default, Component, Reflect, Debug)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Default, Component, Reflect, Debug, Copy, Clone, PartialEq)]
#[reflect(Component)]
pub enum Stance {
    #[default]
    Standing,
    Crouching,
}

#[derive(Default, Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct MovementAcceleration {
    pub ground: f32,
    pub air: f32,
}

impl MovementAcceleration {
    pub fn new(ground: f32, air: f32) -> Self {
        Self { ground, air }
    }
}

#[derive(Default, Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct JumpImpulse(pub f32);

#[derive(Default, Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct MovementDamping {
    pub ground: f32,
    pub air: f32,
}

impl MovementDamping {
    pub fn new(ground: f32, air: f32) -> Self {
        Self { ground, air }
    }
}

#[derive(Default, Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct CrouchModifier(pub f32);

fn check_grounded(mut commands: Commands, player: Single<(Entity, &ShapeHits), With<Player>>) {
    let (entity, hits) = player.into_inner();

    let is_grounded = hits.iter().any(|_hit| return true);

    if is_grounded {
        commands.entity(entity).insert(Grounded);
    } else {
        commands.entity(entity).remove::<Grounded>();
    }
}

fn apply_player_movement_damping(
    player: Single<(&mut LinearVelocity, &MovementDamping, Has<Grounded>), With<Player>>,
) {
    let (mut velocity, movement_damping, is_grounded) = player.into_inner();

    if is_grounded {
        velocity.x *= movement_damping.ground;
        velocity.z *= movement_damping.ground;
    } else {
        velocity.x *= movement_damping.air;
        velocity.z *= movement_damping.air;
    }
}

fn apply_player_movement(
    trigger: Trigger<Fired<Move>>,
    player: Single<
        (
            &mut LinearVelocity,
            &MovementAcceleration,
            &Rotation,
            &Stance,
            Has<Grounded>,
        ),
        With<Player>,
    >,
) {
    let (mut velocity, movement_acceleration, rotation, stance, is_grounded) = player.into_inner();
    let mut movement = trigger.value;

    let movement_modifier = match stance {
        Stance::Standing => 1.0,
        Stance::Crouching => 0.5,
    };

    if is_grounded {
        movement *= movement_acceleration.ground * movement_modifier;
        **velocity += rotation.0 * Vec3::new(movement.x, 0.0, -movement.y);
    } else {
        movement *= movement_acceleration.air * movement_modifier;
        **velocity += rotation.0 * Vec3::new(movement.x, 0.0, -movement.y);
    }
}

fn apply_player_jump(
    trigger: Trigger<Fired<Jump>>,
    player: Single<(&mut LinearVelocity, &JumpImpulse, Has<Grounded>), With<Player>>,
) {
    let (mut velocity, jump_impulse, is_grounded) = player.into_inner();
    let jump = trigger.value;

    if is_grounded {
        if jump {
            velocity.y += jump_impulse.0;
        }
    }
}

fn apply_player_stance(
    actions: Single<&Actions<PlayerActions>>,
    player: Single<&mut Stance, With<Player>>,
) {
    let mut stance = player.into_inner();
    let actions = actions.into_inner();
    let prev_stance = stance.clone();

    if actions.action::<Crouch>().state() == ActionState::Fired {
        *stance = Stance::Crouching;
    } else {
        *stance = Stance::Standing;
    }

    if *stance != prev_stance {
        match *stance {
            Stance::Crouching => {}
            Stance::Standing => {}
        }
    }
}
