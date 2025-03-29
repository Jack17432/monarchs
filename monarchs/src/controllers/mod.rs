use bevy::app::plugin_group;

pub mod player;

plugin_group! {
    #[derive(Debug, Default)]
    pub struct ControllerPluginGroup {
        player:::PlayerControllerPlugin,
    }
}
