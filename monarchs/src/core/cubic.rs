use std::collections::HashMap;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;

#[derive(Default, Debug)]
pub struct CubicPlugin;

impl Plugin for CubicPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Debug, Clone)]
pub struct Block {
    id: u32,
}

#[derive(Debug, Clone)]
pub struct Chunk {
}

impl Chunk {
    pub const SIZE: usize = 16;
}
