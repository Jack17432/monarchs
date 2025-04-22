use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), add_world_lighting);
}

fn add_world_lighting(mut commands: Commands) {
}
