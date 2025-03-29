pub mod player_camera;

use bevy::app::plugin_group;

plugin_group! {
    #[derive(Debug)]
    pub struct ViewsPluginGroup {
        player_camera:::CameraPlugin,
    }
}
