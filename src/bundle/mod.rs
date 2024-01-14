use bevy::prelude::*;

use crate::component;

#[derive(Bundle)]
pub struct PixelBundle{
    pub position: component::Position,
    pub colour: component::Colour
}

#[derive(Bundle)]
pub struct ChunkBundle {
    pub position: component::Position,
    pub entities: component::ChunkList
}
