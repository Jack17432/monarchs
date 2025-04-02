use crate::controllers::player::PlayerControlled;
use crate::core::physics::LinerVelocity;
use crate::views::player_camera::{PlayerCamera, PlayerCameraInfo};
use crate::void_born::vessels::Vessel;
use bevy::prelude::*;

#[derive(Bundle, Debug)]
pub struct SoulBundle {
    soul: Soul,
    next_vessel: NextVessel,
    bound_to_vessel: BoundToVessel,
    owned_vessels: OwnedVessels,
}

impl SoulBundle {
    /// # Panics
    /// Will panic if the entity this soul is bound and next vessel is not in the given vessels.
    pub fn new(bound_to: Entity, next_vessel: Entity, vessels: Vec<Entity>) -> Self {
        assert!(
            vessels.contains(&bound_to),
            "bound vessel: {:?} is not in the supplied list of vessels.",
            bound_to
        );
        assert!(
            vessels.contains(&next_vessel),
            "next vessel: {:?} is not in the supplied list of vessels.",
            bound_to
        );

        Self {
            soul: Soul,
            next_vessel: NextVessel(next_vessel),
            bound_to_vessel: BoundToVessel::new(bound_to),
            owned_vessels: OwnedVessels::new(vessels),
        }
    }
}

#[derive(Component, Debug)]
pub struct Soul;

#[derive(Component, Debug)]
pub struct BoundToVessel(pub Entity);

impl BoundToVessel {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
}

#[derive(Component, Debug)]
pub struct NextVessel(pub Entity);

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum ChangeDir {
    Forward,
    Backward,
}

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct ChangeNextVesselEvent(pub Entity, pub ChangeDir);

impl NextVessel {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
}

#[derive(Component, Debug)]
pub struct OwnedVessels(pub Vec<Entity>);

impl OwnedVessels {
    pub fn new(entities: Vec<Entity>) -> Self {
        Self(entities)
    }
}

/// Contains the event into of
/// - Soul
/// - Current Vessel
/// - Next Vessel
#[derive(Event, Debug)]
pub struct VesselSwapEvent(pub Entity, pub Entity, pub Entity);

pub fn vessel_swap_system(
    mut commands: Commands,
    mut e_swap: EventReader<VesselSwapEvent>,
    camera: Single<Entity, With<PlayerCamera>>,
    mut q_soul: Query<(&mut BoundToVessel, &mut NextVessel), With<Soul>>,
    mut q_vessels: Query<
        (
            &mut Transform,
            &mut LinerVelocity,
            Has<PlayerControlled>,
            Option<&PlayerCameraInfo>,
        ),
        With<Vessel>,
    >,
) {
    for VesselSwapEvent(soul, curr_vessel, next_vessel) in e_swap.read() {
        let Ok((mut bound, mut next_bound)) = q_soul.get_mut(*soul) else {
            error!(soul = ?soul, "Unable to retrieve soul");
            continue;
        };

        if bound.0 == next_bound.0 {
            error!(soul = ?soul, curr = ?bound, next = ?next_bound, "Current vessels is the same as next vessel");
            continue;
        }

        (*bound, *next_bound) = (BoundToVessel::new(next_bound.0), NextVessel::new(bound.0));

        let Ok(
            [
                (mut transform1, mut lin_vel1, is_player_controlled, camera_info),
                (mut transform2, mut lin_vel2, _, _),
            ],
        ) = q_vessels.get_many_mut([*curr_vessel, *next_vessel])
        else {
            error!(current = ?curr_vessel, next = ?next_vessel, "Unable to retrieve vessels");
            continue;
        };
        info!(
            curr_vessel = ?curr_vessel,
            next_vessel = ?next_vessel,
            is_player_controlled = ?is_player_controlled,
            "Vessel swap event"
        );

        if is_player_controlled {
            commands
                .get_entity(*curr_vessel)
                .unwrap()
                .remove::<PlayerControlled>()
                .remove_children(&[*camera]);
            commands
                .get_entity(*next_vessel)
                .unwrap()
                .insert(PlayerControlled)
                .add_child(*camera);
        }

        if let Some(camera_info) = camera_info {
            commands.entity(*curr_vessel).remove::<PlayerCameraInfo>();
            commands.entity(*next_vessel).insert(camera_info.clone());
        };

        (*lin_vel1, *lin_vel2) = (lin_vel2.clone(), lin_vel1.clone());
        (*transform1, *transform2) = (transform2.clone(), transform1.clone());
    }
}

pub fn vessel_change_next_system(
    mut q_souls: Query<(&BoundToVessel, &mut NextVessel, &OwnedVessels)>,
    mut e_vessel_change_next: EventReader<ChangeNextVesselEvent>,
) {
    for ChangeNextVesselEvent(soul, direction) in e_vessel_change_next.read() {
        let (curr_vessel, mut next_vessel, owned_vessels) = q_souls.get_mut(*soul).unwrap();

        let length = owned_vessels.0.len() as isize;
        let mut idx = owned_vessels.0.binary_search(&next_vessel.0).unwrap() as isize;
        let curr_idx = owned_vessels.0.binary_search(&curr_vessel.0).unwrap() as isize;

        if length <= 2 {
            continue;
        }

        info!(idx = ?idx, curr_idx = ?curr_idx, direction = ?direction);

        match direction {
            ChangeDir::Forward => {
                idx += 1;
                while idx == curr_idx || idx >= length {
                    if idx == curr_idx {
                        idx += 1;
                    } else {
                        idx = 0;
                    }
                }
            }
            ChangeDir::Backward => {
                idx -= 1;
                while idx == curr_idx || idx < 0 {
                    if idx == curr_idx {
                        idx -= 1;
                    } else {
                        idx = length - 1;
                    }
                }
            }
        }

        next_vessel.0 = owned_vessels.0[idx as usize];

        info!(dir = ?direction, new_vessel =? next_vessel, all_vessels = ?owned_vessels, "Vessel changed");
    }
}
