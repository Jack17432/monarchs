pub mod debug_tools;
pub mod world;

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct LookDirection(pub Quat);

#[derive(States, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    Running,
    Paused,
}
