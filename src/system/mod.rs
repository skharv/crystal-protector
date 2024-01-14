use bevy::prelude::*;
use bevy_pixels::prelude::*;

mod game;
mod noise;
mod pixel;
mod spread;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game::setup)
            .add_systems(Startup, noise::generate.after(game::setup))
            .add_systems(Startup, spread::spawn.after(noise::generate))
            .add_systems(Update, spread::movement)
            .add_systems(Update, spread::spread)
            .add_systems(Draw, pixel::clear)
            .add_systems(Draw, pixel::draw.after(pixel::clear));
    }
}
