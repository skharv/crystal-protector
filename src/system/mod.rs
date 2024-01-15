use bevy::prelude::*;
use bevy_pixels::prelude::*;

mod game;
mod noise;
mod pixel;
mod player;
mod spread;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, game::setup)
            .add_systems(Startup, noise::generate)
            .add_systems(PostStartup, spread::spawn)
            .add_systems(PostStartup, player::spawn)
            .add_systems(Update, spread::movement)
            .add_systems(Update, spread::spread)
            .add_systems(Update, player::update_input)
            .add_systems(Update, player::update_velocity.after(player::update_input))
            .add_systems(Update, player::update_position.after(player::update_velocity))
            .add_systems(Draw, pixel::clear)
            .add_systems(Draw, pixel::draw.after(pixel::clear));
    }
}
