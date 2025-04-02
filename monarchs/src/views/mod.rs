pub mod graphics;
pub mod player_camera;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(Debug)]
pub struct ViewsPluginGroup;

impl PluginGroup for ViewsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(graphics::GraphicsPluginGroup)
            .add(player_camera::CameraPlugin)
    }
}
