mod camera;
mod character;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(camera::plugin)
        .add_plugins(character::plugin);
}
