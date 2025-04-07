use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    id: f32,
}

#[derive(Debug)]
pub struct Chunk(pub HashMap<(u32, u32, u32), Block>);

impl Chunk {
    pub fn get_block_at(&self, position: &Vec3) -> Option<Block> {
        let Vec3 { x, y, z } = position.round();
        self.0.get(&(x as u32, y as u32, z as u32)).cloned()
    }
}
