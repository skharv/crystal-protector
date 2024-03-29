use bevy::prelude::*;

use crate::utils;

#[derive(Component)]
pub struct Input {
    pub angle: Option<f32>
}

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Speed {
    pub value: f32
}

#[derive(Component)]
pub struct Absorb {
    pub radius: f32,
    pub range: f32
}

#[derive(Component)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Component)]
pub struct Spread {
    pub duration: f32,
    pub counter: f32
}

#[derive(Component)]
pub struct Hunger {
    pub duration: f32,
    pub counter: f32
}

#[derive(Component)]
pub struct Chunk {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct EntityList {
    pub entities: Vec<Entity>
}

#[derive(Component)]
pub struct Land;

#[derive(Component)]
pub struct Floor;

#[derive(Component)]
pub struct Beam;

#[derive(Component)]
pub struct Timer {
    pub remaining: f32
}

#[derive(Component)]
pub struct DeathTimer {
    pub remaining: f32
}

#[derive(Component)]
pub struct Resources {
    pub maximum: i32,
    pub amount: i32
}

#[derive(Component)]
pub struct Ui;

#[derive(Component)]
pub struct Symbol {
    pub shape: String   
}

#[derive(Component)]
pub struct Bar {
    pub percent: f32,
    pub cost: f32
}

#[derive(Component)]
pub struct Size {
    pub width: i32,
    pub height: i32
}

#[derive(Component)]
pub struct Action {
    pub action: utils::Action
}

#[derive(Component)]
pub struct Resource {
    pub value: i32
}

#[derive(Component)]
pub struct ActionUi;

#[derive(Component)]
pub struct Bubble;

#[derive(Component)]
pub struct Finder {
    pub minimum_radius: i32,
    pub maximum_radius: i32,
    pub timer_target: f32,
    pub timer_counter: f32
}

#[derive(Component)]
pub struct Circle {
    pub radius: f32,
}

#[derive(Component)]
pub struct Flicker;

#[derive(Component)]
pub struct Seek {
    pub radius: f32,
    pub entity: Entity
} 

#[derive(Component)]
pub struct Automaton;

#[derive(Component)]
pub struct Indestructable;

#[derive(Component)]
pub struct Bomb {
    pub radius: f32
}

#[derive(Component)]
pub struct Crystal {
    pub id: i32,
}

#[derive(Component)]
pub struct Victory;

#[derive(Component)]
pub struct Defeat;

#[derive(Component)]
pub struct Music;

#[derive(Component)]
pub struct Start;

#[derive(Component)]
pub struct Dying;
