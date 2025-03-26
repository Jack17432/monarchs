pub mod core;
pub mod debug;
pub mod environment;
pub mod views;

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct LookDirection(pub Quat);

#[derive(Component)]
pub struct Player;

#[derive(States, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    Running,
    Paused,
}
