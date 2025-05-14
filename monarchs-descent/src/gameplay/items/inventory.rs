use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Inventory>().register_type::<EquippedItem>();
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Inventory {
    items: Vec<Option<Entity>>,
    capacity: usize,
    count: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: vec![None; capacity],
            capacity,
            count: 0,
        }
    }

    #[allow(dead_code)]
    pub fn from_items(capacity: usize, mut items: Vec<Option<Entity>>) -> Self {
        assert!(
            items.len() <= capacity,
            "capacity to small for initial items"
        );

        let count = items.iter().filter(|&i| i.is_some()).count();
        items.append(&mut vec![None; capacity - items.len()]);

        Self {
            items,
            capacity,
            count,
        }
    }

    pub fn get_items(&self) -> &Vec<Option<Entity>> {
        &self.items
    }

    pub fn insert_item(&mut self, item: Entity) -> Result<(), ()> {
        for mut position in self.items.iter_mut() {
            if position.is_none() {
                *position = Some(item);
                return Ok(());
            }
        }
        Err(())
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct EquippedItem(pub Option<Entity>);

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Equipped;
