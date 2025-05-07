use bevy::ecs::component::{Immutable, StorageType};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Inventory>().register_type::<ItemOf>();
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = ItemOf)]
pub(super) struct Inventory {
    #[relationship]
    items: Vec<Entity>,
    capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: vec![Entity::PLACEHOLDER; capacity],
            capacity,
        }
    }
}

#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
#[relationship(relationship_target = Inventory)]
pub(super) struct ItemOf(pub Entity);

impl Component for ItemOf {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Immutable;
}
