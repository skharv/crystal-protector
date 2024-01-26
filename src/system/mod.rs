use bevy_pixels::prelude::*;
use bevy::prelude::*;
use crate::AppState;

mod action;
mod audio;
mod base;
mod beam;
mod game;
mod noise;
mod pixel;
mod player;
mod spread;
mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, game::setup)
            .add_systems(Startup, (
                    noise::generate, ui::generate_symbols
                        ))
            .add_systems(PostStartup, (
                    base::spawn,
                    game::spawn_crystals,
                        ))
            .add_systems(PostStartup, 
                         (spread::spawn, player::spawn).after(base::spawn)
                    )
            .add_systems(Update, game::start_action.run_if(in_state(AppState::Start)))
            .add_systems(OnEnter(AppState::Game), audio::play_music)
            .add_systems(Update, (
                    spread::movement,
                    spread::spread,
                    spread::hunger,
                    player::update_input,
                    player::absorb,
                    player::update_velocity.after(player::update_input),
                    player::update_position.after(player::update_velocity),
                    player::swap_action,
                    player::action,
                    action::bubble,
                    action::automaton,
                    action::bomb,
                    beam::timer,
                    game::death_timer,
                    game::update_finder,
                    game::depower_crystal,
                    game::lose_game,
                    game::win_game,
                    ui::update_bars,
                    ui::update_action
                    ).run_if(in_state(AppState::Game))
                        )
            .add_systems(Draw, (
                    pixel::clear, 
                    pixel::draw.after(pixel::clear).run_if(in_state(AppState::Game)),
                    pixel::draw_victory.after(pixel::clear).run_if(in_state(AppState::Win)),
                    pixel::draw_defeat.after(pixel::clear).run_if(in_state(AppState::Loss))
                    ));

    }
}
