use bevy::prelude::*;

#[derive(Bundle, Debug)]
pub struct VesselBundle {
    vessel: Vessel,
    name: VesselName,
}

impl VesselBundle {
    pub fn new(name: String, nickname: Option<String>) -> Self {
        Self {
            vessel: Vessel,
            name: VesselName { name, nickname },
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Vessel;

#[derive(Component, Debug)]
pub struct VesselName {
    pub name: String,
    pub nickname: Option<String>,
}

impl VesselName {
    pub fn get_name(&self) -> String {
        if self.nickname.is_some() {
            return self.nickname.clone().unwrap();
        }

        self.name.clone()
    }
}
