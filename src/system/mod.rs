use bevy::prelude::*;
use bevy_pixels::prelude::*;

mod noise;
mod pixel;
mod spread;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, noise::generate)
            .add_systems(Startup, spread::spawn)
            .add_systems(Update, spread::grow)
            .add_systems(Update, spread::spread)
            .add_systems(Draw, pixel::clear)
            .add_systems(Draw, pixel::draw.after(pixel::clear));
    }
}
