use std::collections::HashMap;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;

#[derive(Default, Debug)]
pub struct CubicPlugin;

impl Plugin for CubicPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Default, Debug, Clone)]
pub struct Block {
    id: u32,
}

#[derive(Default, Debug, Resource, ExtractResource, Clone)]
pub struct WorldBlocks(pub HashMap<(isize, isize), Block>);
