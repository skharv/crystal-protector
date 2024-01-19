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
pub struct AutomatonBundle{
    pub position: component::Position,
    pub velocity: component::Velocity,
    pub speed: component::Speed,
    pub colour: component::Colour,
    pub timer: component::DeathTimer,
    pub automaton: component::Automaton,
}

#[derive(Bundle)]
pub struct PlayerBundle{
    pub position: component::Position,
    pub velocity: component::Velocity,
    pub speed: component::Speed,
    pub colour: component::Colour,
    pub input: component::Input,
    pub absorb: component::Absorb,
    pub action: component::Action,
    pub resources: component::Resources
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

#[derive(Bundle)]
pub struct BeamBundle {
    pub position: component::Position,
    pub colour: component::Colour,
    pub timer: component::Timer,
    pub beam: component::Beam
}

#[derive(Bundle)]
pub struct SymbolBundle {
    pub position: component::Position,
    pub colour: component::Colour,
    pub symbol: component::Symbol,
    pub ui: component::Ui
}

#[derive(Bundle)]
pub struct BarBundle {
    pub position: component::Position,
    pub colour: component::Colour,
    pub bar: component::Bar,
    pub size: component::Size,
    pub ui: component::Ui
}

#[derive(Bundle)]
pub struct FinderBundle {
    pub position: component::Position,
    pub colour: component::Colour,
    pub circle: component::Circle,
    pub finder: component::Finder,
}

#[derive(Bundle)]
pub struct BubbleBundle {
    pub circle: component::Circle,
    pub position: component::Position,
    pub timer: component::DeathTimer,
    pub colour: component::Colour,
    pub bubble: component::Bubble
}
