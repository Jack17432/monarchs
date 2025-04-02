pub mod config;
pub mod controllers;
pub mod core;
pub mod debug;
pub mod environment;
pub mod ui;
pub mod views;
pub mod void_born;

use crate::config::ConfigPlugin;
use crate::controllers::ControllerPluginGroup;
use crate::core::CorePluginGroup;
use crate::debug::DebugToolsPlugin;
use crate::environment::WorldPlugin;
use crate::views::ViewsPluginGroup;
use crate::void_born::VoidBornPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(States, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    Running,
    Paused,
}

#[derive(Debug, Default)]
pub struct MonarchsGamePluginGroup;

impl PluginGroup for MonarchsGamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(DebugToolsPlugin)
            .add(ConfigPlugin)
            .add(WorldPlugin)
            .add(VoidBornPlugin)
            .add(ui::UiPlugin)
            .add_group(ViewsPluginGroup)
            .add_group(ControllerPluginGroup)
            .add_group(CorePluginGroup)
    }
}
