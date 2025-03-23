pub mod debug_tools;
pub mod world;

use bevy::prelude::States;

#[derive(States, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    Running,
    Paused,
}
