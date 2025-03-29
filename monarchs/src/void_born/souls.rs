use crate::controllers::player::PlayerControlled;
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
    mut q_vessels: Query<(&mut Transform, Has<PlayerControlled>, Option<&PlayerCameraInfo>), With<Vessel>>,
) {
    for VesselSwapEvent(soul, curr_vessel, next_vessel) in e_swap.read() {
        let Ok((mut bound, mut next_bound)) = q_soul.get_mut(*soul) else {
            error!(soul = ?soul, "Unable to retrieve soul");
            return;
        };

        if bound.0 == next_bound.0 {
            warn!(soul = ?soul, curr = ?bound, next = ?next_bound, "Vessel does not overlap");
            continue;
        }

        (*bound, *next_bound) = (BoundToVessel::new(next_bound.0), NextVessel::new(bound.0));

        let Ok([(mut transform1, is_player_controlled, camera_info), (mut transform2, _, _)]) =
            q_vessels.get_many_mut([*curr_vessel, *next_vessel])
        else {
            error!(current = ?curr_vessel, next = ?next_vessel, "Unable to retrive vessels");
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
            commands
                .entity(*curr_vessel)
                .remove::<PlayerCameraInfo>();
            commands
                .entity(*next_vessel)
                .insert(camera_info.clone());
        };

        (*transform1, *transform2) = (transform2.clone(), transform1.clone());
    }
}
