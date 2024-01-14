use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Component)]
pub struct MoveTimer {
    pub duration: f32,
    pub counter: f32
}

#[derive(Component)]
pub struct Spread {
    pub duration: f32,
    pub counter: f32
}

#[derive(Component)]
pub struct Chunk {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct ChunkList {
    pub entities: Vec<Entity>
}

#[derive(Component)]
pub struct Land;
