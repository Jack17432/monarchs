use crate::gameplay::input::Interact;
use crate::gameplay::items::Item;
use crate::gameplay::items::inventory::{Inventory, ItemOf};
use crate::gameplay::player::Player;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Started;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionRange>();

    app.add_observer(simple_pickup);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub(super) struct InteractionRange(pub f32);

fn simple_pickup(
    _trigger: Trigger<Started<Interact>>,
    player: Single<(Entity, &Transform, &InteractionRange, &mut Inventory), With<Player>>,
    items: Query<(Entity, &Transform, &Item), Without<ItemOf>>,
    mut commands: Commands,
) {
    let (player, player_transform, range, mut inventory) = player.into_inner();
    items.iter().for_each(|(item, transform, item_state)| {
        if (transform.translation - player_transform.translation).length() < range.0 {
            if inventory.insert_item(item).is_err() {
                return;
            };

            commands.entity(player).insert(ItemOf(player));
        }
    })
}
