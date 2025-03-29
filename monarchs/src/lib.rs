pub mod config;
pub mod controllers;
pub mod core;
pub mod debug;
pub mod environment;
pub mod ui;
pub mod views;
pub mod void_born;

use bevy::prelude::*;

#[derive(States, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    Running,
    Paused,
}
