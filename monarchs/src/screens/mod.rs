mod loading;
mod main_menu;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((main_menu::plugin, loading::plugin));
}

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) enum Screen {
    #[default]
    MainMenu,
    Loading,
    Gameplay,
}
