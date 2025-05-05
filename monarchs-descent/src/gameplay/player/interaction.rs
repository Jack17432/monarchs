use crate::gameplay::player::controller::PlayerCamera;
use crate::gameplay::player::Player;
use crate::gameplay::PickupItem;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::egui_utils::easymark::parser::Item;
use bevy_inspector_egui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionRange>();

    app.add_systems(Update, get_interaction_entity);
}

#[derive(Default, Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct InteractionRange(pub f32);

fn get_interaction_entity(
    spatial_query: SpatialQuery,
    camera: Single<&Transform, With<PlayerCamera>>,
    player: Single<&InteractionRange, With<Player>>,
    items: Query<Entity, With<PickupItem>>,
) {
    let binding = spatial_query.ray_hits(
        camera.translation,
        camera.forward(),
        player.0,
        2,
        true,
        &SpatialQueryFilter::default(),
    );
    let Some(out) = binding.get(0) else {
        return;
    };

    if items.contains(out.entity) {
        info!(?out, "Found a item");
    } else {
        info!(?binding, ?out, ?items);
    }
}
