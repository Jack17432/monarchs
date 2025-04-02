pub mod cubic;
pub mod physics;
pub mod solver;

use bevy::app::plugin_group;

plugin_group! {
    pub struct CorePluginGroup {
        solver:::PhysicsSolverPlugin,
        cubic:::CubicPlugin,
    }
}
