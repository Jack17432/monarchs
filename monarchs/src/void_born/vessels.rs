use bevy::prelude::*;

#[derive(Bundle, Debug)]
pub struct VesselBundle {
    vessel: Vessel,
    name: Name,
}

impl VesselBundle {
    pub fn new(name: String, nickname: Option<String>) -> Self {
        Self {
            vessel: Vessel,
            name: Name { name, nickname },
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Vessel;

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
    pub nickname: Option<String>,
}

impl Name {
    pub fn get_name(&self) -> String {
        if self.nickname.is_some() {
            self.nickname.clone().unwrap()
        }

        self.name.clone()
    }
}
