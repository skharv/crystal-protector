use bevy::audio::PlaybackMode;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::bundle;
use crate::component;
use crate::utils;
use crate::AppState;

pub fn setup(
    mut commands: Commands
    ) {
    for x in 0..((crate::WIDTH / crate::SCALE) / crate::CHUNK_SIZE) {
        for y in 0..(((crate::HEIGHT / crate::SCALE) - crate::MENU_SIZE) / crate::CHUNK_SIZE) {
            commands.spawn(bundle::ChunkBundle {
                position: component::Chunk {
                    x,
                    y
                },
                entities: component::EntityList{
                    entities: Vec::new()
                }
            });
        }
    }
}

pub fn spawn_crystals(
    mut commands: Commands,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    land_query: Query<&component::Position, (With<component::Land>, Without<component::Spread>, Without<component::Floor>)>,
    ) {
    for index in 0..utils::CRYSTAL_COUNT {
        let start_x;
        let start_y;

        match index {
            0 => {
                start_x = 50;
                start_y = 50;
            },
            1 => {
                start_x = (crate::WIDTH / crate::SCALE) - 50 - ((crate::SYMBOL_SIZE / 2) as i32);
                start_y = 50;
            },
            2 => {
                start_x = 50;
                start_y = ((crate::HEIGHT / crate::SCALE) - crate::MENU_SIZE) - 50 - ((crate::SYMBOL_SIZE / 2) as i32);
            },
            3 => {
                start_x = (crate::WIDTH / crate::SCALE) - 50 - ((crate::SYMBOL_SIZE / 2) as i32);
                start_y = ((crate::HEIGHT / crate::SCALE) - crate::MENU_SIZE) - 50 - ((crate::SYMBOL_SIZE / 2) as i32);
            },
            _ => continue,
        }

        let outer_crystal_list = utils::convert_string_to_symbol(&utils::SYMBOL_OUTER_CRYSTAL.to_string());
        let inner_crystal_list = utils::convert_string_to_symbol(&utils::SYMBOL_INNER_CRYSTAL.to_string());

        for x in start_x..start_x+crate::SYMBOL_SIZE as i32 {
            for y in start_y..start_y+crate::SYMBOL_SIZE as i32 {
                for (mut list, chunk) in chunk_query.iter_mut() {
                    if chunk.x == (x / crate::CHUNK_SIZE) as i32 && chunk.y == (y / crate::CHUNK_SIZE) as i32 {
                        let mut id = None;
                        let mut spawn = false;
                        if utils::is_position_part_of_symbol(x - start_x, y - start_y, outer_crystal_list) {
                            let mut colour = utils::COLOUR_OUTER_CRYSTAL;
                            if utils::is_position_part_of_symbol(x - start_x, y - start_y, inner_crystal_list) {
                                colour = utils::COLOUR_INNER_CRYSTAL;
                            }
                            id = Some(commands.spawn(bundle::CrystalSectionBundle {
                                position: component::Position { x: x as f32, y: y as f32 },
                                colour: component::Colour { r: colour[0], g: colour[1], b: colour[2], a: colour[3] },
                                crystal: component::Crystal { id: index },
                                land: component::Land
                            }).id());
                            for list_entity in list.entities.iter() {
                                if let Ok(found_entity) = land_query.get(*list_entity) {
                                    if found_entity.x as i32 == x && found_entity.y as i32 == y {
                                        commands.entity(*list_entity).despawn();
                                    }
                                }
                                spawn = true;
                            }
                        }
                        if spawn {
                            if let Some(entity) = id {
                                list.entities.push(entity);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn death_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut component::DeathTimer), Without<component::Bomb>>,
    time: Res<Time>
    ) {
    for (entity, mut timer) in query.iter_mut() {
        timer.remaining -= time.delta_seconds();
        if timer.remaining <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_finder(
    mut commands: Commands,
    mut query: Query<(Entity, &mut component::Finder, &mut component::Circle)>,
    time: Res<Time>
    ) {
    for (entity, mut finder, mut circle) in query.iter_mut() {
        finder.timer_counter += time.delta_seconds();
        let percent = (1.0-(finder.timer_counter / finder.timer_target)) * (finder.maximum_radius - finder.minimum_radius) as f32;
        circle.radius = finder.minimum_radius as f32 + percent;
        if finder.timer_counter >= finder.timer_target {
            commands.entity(entity).despawn();
        }
    }
}

pub fn depower_crystal(
    mut commands: Commands,
    mut query: Query<(Entity, &component::Crystal, &mut component::Colour), Without<component::Ui>>,
    ui_query: Query<(Entity, &component::Crystal), With<component::Ui>>,
    asset_server: Res<AssetServer>
    ) {
    for index in 0..utils::CRYSTAL_COUNT {
        let mut counter = 0;
        for (_, crystal, _) in query.iter() {
            if crystal.id == index {
                counter = counter + 1;
            }
        }
        if counter < utils::CRYSTAL_SECTIONS {
            for (entity, crystal, mut colour) in query.iter_mut() {
                if crystal.id == index {
                    colour.r = utils::COLOUR_SHALLOW[0];
                    colour.g = utils::COLOUR_SHALLOW[1];
                    colour.b = utils::COLOUR_SHALLOW[2];
                    colour.a = utils::COLOUR_SHALLOW[3];
                    commands.entity(entity).remove::<component::Crystal>();
                }
            }
            for (ui_entity, ui_crystal) in ui_query.iter() {
                if ui_crystal.id == index {
                    commands.spawn(AudioBundle{
                        source: asset_server.load("shatter.ogg"),
                        settings: PlaybackSettings{
                            mode: PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    });
                    commands.entity(ui_entity).despawn();
                    return;
                }
            }
        }
    }
}

pub fn lose_game(
    mut commands: Commands,
    ui_query: Query<(Entity, &component::Crystal), With<component::Ui>>,
    bgm_query: Query<(Entity, &AudioSink), With<component::Music>>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>
    ) {
    if ui_query.iter().count() == 0 {
        if let Ok((entity, sink)) = bgm_query.get_single(){
            sink.stop();
            commands.entity(entity).despawn();
            commands.spawn(AudioBundle{
                source: asset_server.load("lose.ogg"),
                settings: PlaybackSettings{
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
                ..default()
            });
        }
        app_state.set(AppState::Loss);
    }
}

pub fn start_action(
    mut keys: EventReader<KeyboardInput>,
    mut button: EventReader<MouseButtonInput>,
    mut app_state: ResMut<NextState<AppState>>
    ) {
    for ev in keys.read() {
        match ev.state {
            ButtonState::Pressed => {
            }
            ButtonState::Released => {
                app_state.set(AppState::Game);
            }
        }
    }
    for ev in button.read() {
        match ev.state {
            ButtonState::Pressed => {
            }
            ButtonState::Released => {
                app_state.set(AppState::Game);
            }
        }
    }
}

pub fn win_game(
    mut commands: Commands,
    query: Query<Entity, With<component::Spread>>,
    bgm_query: Query<(Entity, &AudioSink), With<component::Music>>,
    mut app_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>
    ) {
    if query.iter().count() == 0 {
        if let Ok((entity, sink)) = bgm_query.get_single(){
            sink.stop();
            commands.entity(entity).despawn();
            commands.spawn(AudioBundle{
                source: asset_server.load("win.ogg"),
                settings: PlaybackSettings{
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
                ..default()
            });
        }
        app_state.set(AppState::Win);
    }
}
