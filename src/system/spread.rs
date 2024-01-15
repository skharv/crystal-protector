use bevy::prelude::*;
use rand::Rng;

use crate::bundle;
use crate::component;

const SPREAD: [u8; 4] = [148, 197, 172, 255];

pub fn spawn(
    mut commands: Commands,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    land_query: Query<&component::Position, (With<component::Land>, Without<component::Spread>)>,
    ) {
    let mut rng = rand::thread_rng();

    let mut x = rng.gen_range(0..crate::WIDTH/crate::SCALE);
    let mut y = rng.gen_range(0..crate::HEIGHT/crate::SCALE);

    while land_query.iter().any(|pos| pos.x as i32 == x as i32 && pos.y as i32 == y as i32) {
        x = rng.gen_range(0..crate::WIDTH/crate::SCALE);
        y = rng.gen_range(0..crate::HEIGHT/crate::SCALE);
    }

    let new_angle = rng.gen_range(0.0..360.0); 
    let entity = commands.spawn(bundle::SpreadBundle {
        position: component::Position { x: x as f32, y: y as f32 },
        velocity: component::Velocity { x: f32::cos(new_angle), y:f32::sin(new_angle) },
        speed: component::Speed { value: rng.gen_range(0.5..5.0) },
        colour: component::Colour { r: SPREAD[0], g: SPREAD[1], b: SPREAD[2], a: SPREAD[3] },
        spread: component::Spread { duration: 5.0, counter: 0.0 }
    }).id();

    for (mut list, chunk) in chunk_query.iter_mut() {
        if chunk.x == (x / crate::CHUNK_SIZE) && chunk.y == (y / crate::CHUNK_SIZE) {
            list.entities.push(entity);
        }
    }
}

pub fn movement(
    mut commands: Commands,
    mut spread_query: Query<(Entity, &mut component::Position, &mut component::Velocity), With<component::Spread>>,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    land_query: Query<&component::Position, (With<component::Land>, Without<component::Spread>)>,
    time: Res<Time>
    ) {
    let mut rng = rand::thread_rng();
    for (entity, mut position, mut velocity) in spread_query.iter_mut() {

        let mut new_position = Vec2::new(position.x + (velocity.x * time.delta_seconds()), position.y + (velocity.y * time.delta_seconds()));
        let mut update_angle = false;

        if new_position.x > (crate::WIDTH / crate::SCALE) as f32 {
            new_position.x = (crate::WIDTH / crate::SCALE)as f32;
            update_angle = true;
        }
        if new_position.x < 0.0 {
            new_position.x = 0.0;
            update_angle = true;
        }
        if new_position.y > (crate::HEIGHT / crate::SCALE) as f32 {
            new_position.y = (crate::HEIGHT / crate::SCALE) as f32;
            update_angle = true;
        }
        if new_position.y < 0.0 {
            new_position.y = 0.0;
            update_angle = true;
        }

        let mut update_position = true;

        for (list, chunk) in chunk_query.iter_mut() {
            if chunk.x - (new_position.x as i32 / crate::CHUNK_SIZE) > -1 && chunk.x - (new_position.x as i32 / crate::CHUNK_SIZE) < 1 {
                if chunk.y - (new_position.y as i32 / crate::CHUNK_SIZE) > -1 && chunk.y - (new_position.y as i32 / crate::CHUNK_SIZE) < 1{
                    for list_entity in list.entities.iter() {
                        if let Ok(found_entity) = land_query.get(*list_entity) {
                            if found_entity.x as i32 == new_position.x as i32 && found_entity.y as i32 == new_position.y as i32 {
                                update_angle = true;
                                update_position = false;
                                commands.entity(*list_entity).despawn();
                                break;
                            }
                        }
                    }
                }
            }
        }

        if update_angle {
            let new_angle = rng.gen_range(0.0..360.0); 
            velocity.x = f32::cos(new_angle);
            velocity.y = f32::sin(new_angle);
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
                            list.entities.swap_remove(index);
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

pub fn spread(
    mut commands: Commands,
    mut spread_query: Query<(&component::Position, &component::Colour, &mut component::Spread)>,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    time: Res<Time>
    ) {
    let mut rng = rand::thread_rng();
    for (position, spread_colour, mut spread) in spread_query.iter_mut() {
        spread.counter += time.delta_seconds();

        if spread.counter >= spread.duration {
            spread.counter = 0.0;

            let new_position = Vec2::new(position.x, position.y);
            let new_angle = rng.gen_range(0.0..360.0); 

            let entity = commands.spawn(bundle::SpreadBundle {
                position: component::Position { x: new_position.x as f32, y: new_position.y as f32 },
                velocity: component::Velocity { x: f32::cos(new_angle), y:f32::sin(new_angle) },
                speed: component::Speed { value: rng.gen_range(0.5..5.0) },
                colour: component::Colour { r: spread_colour.r, g: spread_colour.g, b: spread_colour.b, a: spread_colour.a },
                spread: component::Spread { duration: rng.gen_range(5.0..20.0), counter: 0.0 }
            }).id();

            for (mut list, chunk) in chunk_query.iter_mut() {
                if chunk.x == (new_position.x as i32 / crate::CHUNK_SIZE) && chunk.y == (new_position.y as i32 / crate::CHUNK_SIZE) {
                    list.entities.push(entity);
                }
            }

            spread.duration *= 2.0;
        }
    }
}

