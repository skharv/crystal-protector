use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::utils;
use crate::bundle;
use crate::component;

const SPAWN_X: f32 = (crate::WIDTH/crate::SCALE)as f32 * 0.5;
const SPAWN_Y: f32 = ((crate::HEIGHT/crate::SCALE) - crate::MENU_SIZE) as f32 * 0.5;

pub fn spawn(
    mut commands: Commands,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    ) {
    let entity = commands.spawn(bundle::PlayerBundle {
        position: component::Position { x: SPAWN_X, y: SPAWN_Y },
        velocity: component::Velocity { x: 0.0, y: 0.0 },
        speed: component::Speed { value: 20.0 },
        colour: component::Colour { r: utils::COLOUR_PLAYER[0], g: utils::COLOUR_PLAYER[1], b: utils::COLOUR_PLAYER[2], a: utils::COLOUR_PLAYER[3] },
        input: component::Input { angle: None },
        absorb: component::Absorb { radius: 2.5, range: 20.0 },
        action: component::Action { action: utils::Action::House },
        resources: component::Resources { maximum: 1000, amount: 0 }
    }).id();

    commands.spawn(bundle::FinderBundle {
        position: component::Position { x: SPAWN_X, y: SPAWN_Y },
        colour: component::Colour { r: utils::COLOUR_PLAYER[0], g: utils::COLOUR_PLAYER[1], b: utils::COLOUR_PLAYER[2], a: utils::COLOUR_PLAYER[3] },
        circle: component::Circle { radius: 100.0 },
        finder: component::Finder { minimum_radius: 5, maximum_radius: 100, timer_target: 0.75, timer_counter: 0.0 },
    });

    for (mut list, chunk) in chunk_query.iter_mut() {
        if chunk.x == (SPAWN_X as i32 / crate::CHUNK_SIZE) && chunk.y == (SPAWN_Y as i32 / crate::CHUNK_SIZE) {
            list.entities.push(entity);
        }
    }
}

pub fn update_velocity(
    mut query: Query<(&mut component::Velocity, &component::Input, &component::Speed), Without<component::Dying>>
    ) {
    for (mut velocity, input, speed) in query.iter_mut() {
        if let Some(direction) = input.angle {
            velocity.x = f32::sin(direction) * -speed.value;
            velocity.y = f32::cos(direction) * -speed.value;
        } else {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

pub fn update_position(
    mut query: Query<(Entity, &mut component::Position, &component::Velocity), With<component::Input>>,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    land_query: Query<&component::Position, (With<component::Land>, Without<component::Input>, Without<component::Spread>, Without<component::Dying>)>,
    time: Res<Time>
    ) {
    for (entity, mut position, velocity) in query.iter_mut() {
        let mut new_position = Vec2::new(position.x + (velocity.x * time.delta_seconds()), position.y + (velocity.y * time.delta_seconds()));

        if new_position.x.is_nan() || new_position.y.is_nan() {
            continue;
        }

        if new_position.x > (crate::WIDTH / crate::SCALE) as f32 {
            new_position.x = (crate::WIDTH / crate::SCALE)as f32 - 1.0;
        }
        if new_position.x < 0.0 {
            new_position.x = 0.0;
        }
        if new_position.y > ((crate::HEIGHT / crate::SCALE) - crate::MENU_SIZE) as f32 {
            new_position.y = ((crate::HEIGHT / crate::SCALE) - crate::MENU_SIZE) as f32 - 1.0;
        }
        if new_position.y < 0.0 {
            new_position.y = 0.0;
        }

        let mut update_position = true;

        for (list, chunk) in chunk_query.iter_mut() {
            if chunk.x - (new_position.x as i32 / crate::CHUNK_SIZE) > -1 && chunk.x - (new_position.x as i32 / crate::CHUNK_SIZE) < 1 {
                if chunk.y - (new_position.y as i32 / crate::CHUNK_SIZE) > -1 && chunk.y - (new_position.y as i32 / crate::CHUNK_SIZE) < 1{
                    for list_entity in list.entities.iter() {
                        if let Ok(found_entity) = land_query.get(*list_entity) {
                            if found_entity.x as i32 == new_position.x as i32 && found_entity.y as i32 == new_position.y as i32 {
                                update_position = false;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if update_position {
            let old_chunk_x = position.x as i32 / crate::CHUNK_SIZE;
            let old_chunk_y = position.y as i32 / crate::CHUNK_SIZE;
            let new_chunk_x = new_position.x as i32 / crate::CHUNK_SIZE;
            let new_chunk_y = new_position.y as i32 / crate::CHUNK_SIZE;

            if old_chunk_x != new_chunk_x || old_chunk_y != new_chunk_y {
                for (mut list, chunk) in chunk_query.iter_mut() {
                    if chunk.x == old_chunk_x && chunk.y == old_chunk_y {
                        if let Some(index) = list.entities.iter().position(|i| *i == entity) {
                            list.entities.remove(index);
                        }
                    }
                    if chunk.x == new_chunk_x && chunk.y == new_chunk_y {
                        list.entities.push(entity);
                    }
                }
            }
            position.x = new_position.x;
            position.y = new_position.y;
        }
    }
}


pub fn update_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut component::Input>
    ) {
    for mut input in query.iter_mut() {
        let mut vector = Vec2::new(0.0, 0.0);
        let mut be_none = true;
        if keys.pressed(KeyCode::W) {
            vector.y += 1.0;
            be_none = false;
        }
        if keys.pressed(KeyCode::A) {
            vector.x += 1.0;
            be_none = false;
        }
        if keys.pressed(KeyCode::S) {
            vector.y -= 1.0;
            be_none = false;
        }
        if keys.pressed(KeyCode::D) {
            vector.x -= 1.0;
            be_none = false;
        }
        vector = vector.normalize();
        if be_none {
            input.angle = None;
        } else {
            input.angle = Some(vector.x.atan2(vector.y));
        }
    }
}

pub fn absorb(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut absorb_query: Query<(&component::Position, &mut component::Resources, &component::Absorb), (With<component::Input>, Without<component::Dying>)>,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    mut land_query: Query<(&component::Position, &mut component::Colour, Option<&component::Resource>, Option<&component::Indestructable>), (With<component::Land>, Without<component::Input>, Without<component::Spread>, Without<component::DeathTimer>, Without<component::Dying>)>,
    asset_server: Res<AssetServer>
    ) {
    let mut rng = rand::thread_rng();
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.single().cursor_position() {
            commands.spawn(AudioBundle {
                        source: asset_server.load("laser.ogg"),
                        settings: PlaybackSettings{
                            mode: PlaybackMode::Despawn,
                            volume: Volume::new_relative(0.25),
                            ..default()
                        },
                        ..default()
                    });
            for (position, mut resources, absorb) in absorb_query.iter_mut() { 
                let distance = ((cursor_position.x/2.0 - position.x).powi(2) + (cursor_position.y/2.0 - position.y).powi(2)).sqrt();
                for step in 1..absorb.range as i32 {
                    let x = position.x + (((step as f32) * (cursor_position.x/2.0 - position.x)) / distance);
                    let y = position.y + (((step as f32) * (cursor_position.y/2.0 - position.y)) / distance);
                    commands.spawn(bundle::BeamBundle {
                        position: component::Position{ x, y },
                        colour: component::Colour{ r: utils::COLOUR_BEAM[0], g: utils::COLOUR_BEAM[1], b: utils::COLOUR_BEAM[2], a: utils::COLOUR_BEAM[3]},
                        timer: component::Timer{ remaining: rng.gen_range(0.05..0.25) },
                        beam: component::Beam
                    });
                    let mut target_position = Vec2::new(0.0, 0.0);
                    let mut target_chunk = IVec2::new(0, 0);
                    let mut target_hit = false;

                    'finder: for (list, chunk) in chunk_query.iter_mut() {
                        if chunk.x - (x as i32 / crate::CHUNK_SIZE) > -1 && chunk.x - (x as i32 / crate::CHUNK_SIZE) < 1 {
                            if chunk.y - (y as i32 / crate::CHUNK_SIZE) > -1 && chunk.y - (y as i32 / crate::CHUNK_SIZE) < 1{
                                for list_entity in list.entities.iter() {
                                    if let Ok((found_entity, _, _, _)) = land_query.get_mut(*list_entity) {
                                        if found_entity.x as i32 == x as i32 && found_entity.y as i32 == y as i32 {
                                            target_hit = true;
                                            target_position = Vec2::new(found_entity.x, found_entity.y);
                                            target_chunk = IVec2::new(chunk.x, chunk.y);
                                            break 'finder;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if target_hit {
                        for (list, chunk) in chunk_query.iter_mut() {
                            if chunk.x - target_chunk.x > -2 && chunk.x - target_chunk.x < 2 {
                                if chunk.y - target_chunk.y > -2 && chunk.y - target_chunk.y < 2{
                                    for list_entity in list.entities.iter() {
                                        if let Ok((found_entity, mut found_colour, found_resource, found_indestructable)) = land_query.get_mut(*list_entity) {
                                            if let None = found_indestructable {
                                                let distance = ((found_entity.x - target_position.x).powi(2) + (found_entity.y - target_position.y).powi(2)).sqrt();
                                                if distance <= absorb.radius {
                                                    found_colour.r = utils::COLOUR_BEAM[0];
                                                    found_colour.g = utils::COLOUR_BEAM[1];
                                                    found_colour.b = utils::COLOUR_BEAM[2];
                                                    found_colour.a = utils::COLOUR_BEAM[3];
                                                    commands.entity(*list_entity).insert(component::DeathTimer{ remaining: rng.gen_range(0.05..0.25) });
                                                    if let Some(resource) = found_resource {
                                                        if resources.amount < resources.maximum {
                                                            resources.amount = (resources.amount + resource.value).min(resources.maximum);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        return;
                    }
                }
            }
        }
    }
}

pub fn swap_action(
    keys: Res<Input<KeyCode>>,
    mut action_query: Query<&mut component::Action, With<component::Input>>,
    ) {
    if keys.just_pressed(KeyCode::Tab) {
        for mut action in action_query.iter_mut() {
            match action.action {
                utils::Action::House => action.action = utils::Action::Bomb,
                utils::Action::Bomb => action.action = utils::Action::Face,
                utils::Action::Face => action.action = utils::Action::Bubble,
                utils::Action::Bubble => action.action = utils::Action::House,
            }
        }
    }
    if keys.just_pressed(KeyCode::Key1) {
        for mut action in action_query.iter_mut() {
            action.action = utils::Action::House;
        }
    }
    if keys.just_pressed(KeyCode::Key2) {
        for mut action in action_query.iter_mut() {
            action.action = utils::Action::Bomb;
        }
    }
    if keys.just_pressed(KeyCode::Key3) {
        for mut action in action_query.iter_mut() {
            action.action = utils::Action::Face;
        }
    }
    if keys.just_pressed(KeyCode::Key4) {
        for mut action in action_query.iter_mut() {
            action.action = utils::Action::Bubble;
        }
    }
}

pub fn action(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut action_query: Query<(&mut component::Position, &mut component::Resources, &component::Speed, &component::Action), With<component::Input>>,
    asset_server: Res<AssetServer>
    ) {
    let mut rng = rand::thread_rng();

    if buttons.just_pressed(MouseButton::Right) {
        if let Some(cursor_position) = window.single().cursor_position() {
            for (mut position, mut resources, speed, action) in action_query.iter_mut() { 
                match action.action {
                    utils::Action::Bomb => {
                        if resources.amount >= utils::Action::Bomb as i32 {
                            let bomb_speed = 10.0 + speed.value;
                            let vel = Vec2::new((cursor_position.x/2.0) - position.x, (cursor_position.y/2.0) - position.y).normalize();

                            commands.spawn(bundle::BombBundle{
                                position: component::Position{ x: position.x, y: position.y },
                                velocity: component::Velocity { x: vel.x, y: vel.y},
                                speed: component::Speed { value: bomb_speed },
                                colour: component::Colour { r: utils::COLOUR_BEAM[0], g: utils::COLOUR_BEAM[1], b: utils::COLOUR_BEAM[2], a: utils::COLOUR_BEAM[3] },
                                timer: component::DeathTimer { remaining: 5.0 },
                                bomb: component::Bomb { radius: 20.0 }
                            });
                            resources.amount -= utils::Action::Bomb as i32;
                       } else {
                            commands.spawn(AudioBundle{
                                source: asset_server.load("error.ogg"),
                                settings: PlaybackSettings {
                                    mode: PlaybackMode::Despawn,
                                    volume: Volume::new_relative(0.5),
                                    ..default()
                                },
                                ..default()
                            });
                        }
                    },
                    utils::Action::Face => {
                        if resources.amount >= utils::Action::Face as i32 {
                            let angle = rng.gen_range(0.0..360.0);
                            let vel = Vec2::new(f32::sin(angle), f32::cos(angle));
                            commands.spawn((bundle::AutomatonBundle {
                                position: component::Position{ x: position.x, y: position.y },
                                velocity: component::Velocity { x: vel.x, y: vel.y },
                                speed: component::Speed { value: 20.0 },
                                colour: component::Colour { r: utils::COLOUR_BEAM[0], g: utils::COLOUR_BEAM[1], b: utils::COLOUR_BEAM[2], a: utils::COLOUR_BEAM[3] },
                                timer: component::DeathTimer { remaining: 30.0 },
                                automaton: component::Automaton
                            },
                            AudioBundle{
                                source: asset_server.load("robot.ogg"),
                                settings: PlaybackSettings{
                                    mode: PlaybackMode::Remove,
                                    ..default()
                                },
                                ..default()
                            }));
                            resources.amount -= utils::Action::Face as i32;
                       } else {
                            commands.spawn(AudioBundle{
                                source: asset_server.load("error.ogg"),
                                settings: PlaybackSettings {
                                    mode: PlaybackMode::Despawn,
                                    volume: Volume::new_relative(0.5),
                                    ..default()
                                },
                                ..default()
                            });
                        }
                    },
                    utils::Action::Bubble => {
                        if resources.amount >= utils::Action::Bubble as i32 {
                            commands.spawn((bundle::BubbleBundle {
                                position: component::Position{ x: position.x, y: position.y },
                                circle: component::Circle { radius: 10.0 },
                                timer: component::DeathTimer { remaining: 60.0 },
                                colour: component::Colour { r: utils::COLOUR_SPREAD[0], g: utils::COLOUR_SPREAD[1], b: utils::COLOUR_SPREAD[2], a: utils::COLOUR_SPREAD[3] },
                                bubble: component::Bubble
                            },
                            AudioBundle{
                                source: asset_server.load("bubble.ogg"),
                                settings: PlaybackSettings{
                                    mode: PlaybackMode::Remove,
                                    ..default()
                                },
                                ..default()
                            }));
                            resources.amount -= utils::Action::Bubble as i32;
                       } else {
                            commands.spawn(AudioBundle{
                                source: asset_server.load("error.ogg"),
                                settings: PlaybackSettings {
                                    mode: PlaybackMode::Despawn,
                                    volume: Volume::new_relative(0.5),
                                    ..default()
                                },
                                ..default()
                            });
                        }
                    },
                    utils::Action::House => {
                        if resources.amount >= utils::Action::House as i32 {
                            commands.spawn(bundle::FinderBundle {
                                position: component::Position{ x: position.x, y: position.y },
                                colour: component::Colour { r: utils::COLOUR_BEAM[0], g: utils::COLOUR_BEAM[1], b: utils::COLOUR_BEAM[2], a: utils::COLOUR_BEAM[3] },
                                circle: component::Circle { radius: 0.0 },
                                finder: component::Finder { minimum_radius: 15, maximum_radius: 0, timer_target: 0.25 , timer_counter: 0.0 }
                            });
                            position.x = SPAWN_X;
                            position.y = SPAWN_Y;
                            commands.spawn(bundle::FinderBundle {
                                position: component::Position{ x: position.x, y: position.y },
                                colour: component::Colour { r: utils::COLOUR_BEAM[0], g: utils::COLOUR_BEAM[1], b: utils::COLOUR_BEAM[2], a: utils::COLOUR_BEAM[3] },
                                circle: component::Circle { radius: 0.0 },
                                finder: component::Finder { minimum_radius: 0, maximum_radius: 15, timer_target: 0.25 , timer_counter: 0.0 }
                            });
                            commands.spawn(AudioBundle{
                                source: asset_server.load("teleport.ogg"),
                                settings: PlaybackSettings {
                                    mode: PlaybackMode::Despawn,
                                    ..default()
                                },
                                ..default()
                            });
                            resources.amount -= utils::Action::House as i32;
                       } else {
                            commands.spawn(AudioBundle{
                                source: asset_server.load("error.ogg"),
                                settings: PlaybackSettings {
                                    mode: PlaybackMode::Despawn,
                                    volume: Volume::new_relative(0.5),
                                    ..default()
                                },
                                ..default()
                            });
                       }
                    },
                }
            }
        }
    }
}

