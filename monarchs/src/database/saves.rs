use crate::database::files;
use bevy::prelude::*;
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(create_new_save).add_observer(load_save);
}

#[derive(Event, Debug)]
pub struct CreateNewSave {
    pub(crate) name: String,
}

#[derive(Event, Debug)]
pub struct LoadSave {
    pub(crate) name: String,
}

fn create_new_save(save_info: Trigger<CreateNewSave>, mut commands: Commands, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Loading);
    
    commands.trigger(LoadSave {
        name: save_info.name.clone(),
    })
}

fn load_save(save_info: Trigger<LoadSave>, mut commands: Commands, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Loading);
}
