pub mod physics;
pub mod solver;
mod world;

use bevy::app::plugin_group;

plugin_group! {
    pub struct CorePluginGroup {
        solver:::PhysicsSolverPlugin,
    }
}
