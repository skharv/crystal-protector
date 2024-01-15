use bevy::prelude::*;

use crate::component;

#[derive(Bundle)]
pub struct PixelBundle{
    pub position: component::Position,
    pub colour: component::Colour
}

#[derive(Bundle)]
pub struct SpreadBundle{
    pub position: component::Position,
    pub velocity: component::Velocity,
    pub speed: component::Speed,
    pub colour: component::Colour,
    pub spread: component::Spread,
    pub hunger: component::Hunger
}

#[derive(Bundle)]
pub struct PlayerBundle{
    pub position: component::Position,
    pub velocity: component::Velocity,
    pub speed: component::Speed,
    pub colour: component::Colour,
    pub input: component::Input
}

#[derive(Bundle)]
pub struct LandBundle{
    pub position: component::Position,
    pub colour: component::Colour,
    pub land: component::Land
}

#[derive(Bundle)]
pub struct ChunkBundle {
    pub position: component::Chunk,
    pub entities: component::EntityList
}
