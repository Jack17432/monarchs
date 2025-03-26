pub mod debug;
pub mod environment;
pub mod core;

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
