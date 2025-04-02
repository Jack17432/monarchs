mod render_blocks;

use bevy::app::plugin_group;

plugin_group! {
    #[derive(Debug, Default)]
    pub struct GraphicsPluginGroup {
        render_blocks:::RenderBlocksPlugin,
    }
}
