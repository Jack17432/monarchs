pub mod blocks;

use crate::core::world::blocks::Chunk;
use bevy::prelude::*;
use std::collections::HashMap;

pub const CHUNK_SIZE: u32 = 16;

#[derive(Component, Debug)]
pub struct World {
    chunks: HashMap<(u32, u32), Chunk>,
}

impl World {
    pub fn get_chunk_at(&self, position: &Vec3) -> Option<&Chunk> {
        let Vec3 { x, y, .. } = round_to_nearest(position, CHUNK_SIZE as f32);

        self.chunks.get(&(x as u32, y as u32))
    }
}

fn round_to_nearest(a: &Vec3, to: f32) -> Vec3 {
    let x = (a.x / to).round() * to;
    let y = (a.y / to).round() * to;
    let z = (a.z / to).round() * to;

    Vec3::new(x, y, z)
}
