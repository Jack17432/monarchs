use bevy::prelude::States;

pub mod start_menu;

#[derive(States, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    Running,
    Paused,
}