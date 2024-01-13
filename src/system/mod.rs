use bevy::prelude::*;
use bevy_pixels::prelude::*;

mod pixel;
mod noise;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, noise::generate)
            .add_systems(Draw, pixel::clear)
            .add_systems(Draw, pixel::draw.after(pixel::clear));
    }
}
