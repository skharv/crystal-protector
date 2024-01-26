use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use rand::Rng;

use crate::bundle;
use crate::component;
use crate::utils;

pub fn bubble (
    mut bubble_query: Query<(&component::Position, &component::Circle), With<component::Bubble>>,
    chunk_query: Query<(&component::EntityList, &component::Chunk)>, 
    mut spread_query: Query<(&component::Position, &mut component::Velocity), With<component::Spread>>,
    ) {
    for (bubble_position, bubble_circle) in bubble_query.iter_mut() {
        let chunk_radius = ((bubble_circle.radius / crate::CHUNK_SIZE as f32).ceil() * 2.0) as i32;
        for (chunk_list, chunk_pos) in chunk_query.iter() {
            if chunk_pos.x - (bubble_position.x as i32 / crate::CHUNK_SIZE) > -chunk_radius && chunk_pos.x - (bubble_position.x as i32 / crate::CHUNK_SIZE) < chunk_radius {
                if chunk_pos.y - (bubble_position.y as i32 / crate::CHUNK_SIZE) > -chunk_radius && chunk_pos.y - (bubble_position.y as i32 / crate::CHUNK_SIZE) < chunk_radius {
                    for list_entity in chunk_list.entities.iter() {
                        if let Ok((spread_position, mut spread_velocity)) = spread_query.get_mut(*list_entity) {
                            let x = bubble_position.x - spread_position.x;
                            let y = bubble_position.y - spread_position.y;
                            let distance = (x.powi(2) + y.powi(2)).sqrt();
                            if distance <= bubble_circle.radius {
                                let angle = y.atan2(x);
                                spread_velocity.x = -angle.cos();
                                spread_velocity.y = -angle.sin();
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn automaton (
    mut commands: Commands,
    mut automaton_query: Query<(Entity, &mut component::Position, &mut component::Velocity, &component::Speed, &component::DeathTimer, Option<&mut component::Seek>), (With<component::Automaton>, Without<component::Land>, Without<component::Spread>)>,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    spread_query: Query<&component::Position, (With<component::Spread>, Without<component::Automaton>, Without<component::Input>)>,
    land_query: Query<&component::Position, (With<component::Land>, Without<component::Spread>, Without<component::Automaton>)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>
    ) {
    let mut rng = rand::thread_rng();
    for (automaton, mut auto_position, mut auto_velocity, auto_speed, auto_death_timer, auto_seek) in automaton_query.iter_mut() {
        if let Some(seek) = auto_seek {
            if let Ok(target_position) = spread_query.get(seek.entity) {
                let x = target_position.x - auto_position.x;
                let y = target_position.y - auto_position.y;
                let angle = y.atan2(x);
                if x.abs() < 1.0 && y.abs() < 1.0 {
                    commands.entity(seek.entity).despawn();
                    commands.entity(automaton).remove::<component::Seek>();
                    commands.spawn(AudioBundle{
                        source: asset_server.load("./robot_attack.ogg"),
                        settings: PlaybackSettings{
                            mode: PlaybackMode::Despawn,
                            ..default()
                        },
                        ..default()
                    });
                }
                auto_velocity.x = angle.cos();
                auto_velocity.y = angle.sin();
            } else {
                commands.entity(automaton).remove::<component::Seek>();
            }
        } else {
            for (mut list, chunk) in chunk_query.iter_mut() {
                if chunk.x - (auto_position.x as i32 / crate::CHUNK_SIZE) > -1 && chunk.x - (auto_position.x as i32 / crate::CHUNK_SIZE) < 1 {
                    if chunk.y - (auto_position.y as i32 / crate::CHUNK_SIZE) > -1 && chunk.y - (auto_position.y as i32 / crate::CHUNK_SIZE) < 1{
                        let mut entity: Option<Entity> = None;
                        let mut distance = f32::MAX;
                        for list_entity in list.entities.iter_mut() {
                            if let Ok(spread_position) = spread_query.get(*list_entity) {
                                let x = auto_position.x - spread_position.x;
                                let y = auto_position.y - spread_position.y;
                                let new_distance = (x.powi(2) + y.powi(2)).sqrt();
                                if new_distance < distance {
                                    distance = new_distance;
                                    entity = Some(*list_entity);
                                }
                            }
                        }
                        if auto_death_timer.remaining > 0.0 {
                            if let Some(closest_entity) = entity { 
                                commands.entity(automaton).insert(component::Seek{ radius: utils::AUTOMATON_SEEK_RANGE, entity: closest_entity });
                            }
                        }
                    }
                }
            }
        }
        let mut new_position = Vec2::new(auto_position.x + (auto_velocity.x * auto_speed.value * time.delta_seconds()), auto_position.y + (auto_velocity.y * auto_speed.value * time.delta_seconds()));
        let mut update_angle = false;
        let mut update_position = true;

        if new_position.x > (crate::WIDTH / crate::SCALE) as f32 {
            new_position.x = (crate::WIDTH / crate::SCALE)as f32 - 1.0;
            update_angle = true;
        }
        if new_position.x < 0.0 {
            new_position.x = 0.0;
            update_angle = true;
        }
        if new_position.y > ((crate::HEIGHT / crate::SCALE) - crate::MENU_SIZE) as f32 {
            new_position.y = ((crate::HEIGHT / crate::SCALE) - crate::MENU_SIZE) as f32 - 1.0;
            update_angle = true;
        }
        if new_position.y < 0.0 {
            new_position.y = 0.0;
            update_angle = true;
        }

        for (list, chunk) in chunk_query.iter_mut() {
            if chunk.x - (new_position.x as i32 / crate::CHUNK_SIZE) > -1 && chunk.x - (new_position.x as i32 / crate::CHUNK_SIZE) < 1 {
                if chunk.y - (new_position.y as i32 / crate::CHUNK_SIZE) > -1 && chunk.y - (new_position.y as i32 / crate::CHUNK_SIZE) < 1{
                    for list_entity in list.entities.iter() {
                        if let Ok(found_entity) = land_query.get(*list_entity) {
                            if found_entity.x as i32 == new_position.x as i32 && found_entity.y as i32 == new_position.y as i32 {
                                update_angle = true;
                                update_position = false;
                            }
                        }
                    }
                }
            }
        }

        if update_angle {
            let new_angle = rng.gen_range(0.0..360.0); 
            auto_velocity.x = f32::cos(new_angle);
            auto_velocity.y = f32::sin(new_angle);
        }

        if update_position {
            let old_chunk_x = auto_position.x as i32 / crate::CHUNK_SIZE;
            let old_chunk_y = auto_position.y as i32 / crate::CHUNK_SIZE;
            let new_chunk_x = new_position.x as i32 / crate::CHUNK_SIZE;
            let new_chunk_y = new_position.y as i32 / crate::CHUNK_SIZE;

            if old_chunk_x != new_chunk_x || old_chunk_y != new_chunk_y {
                for (mut list, chunk) in chunk_query.iter_mut() {
                    if chunk.x == old_chunk_x && chunk.y == old_chunk_y {
                        if let Some(index) = list.entities.iter().position(|i| *i == automaton) {
                            list.entities.swap_remove(index);
                        }
                    }
                    if chunk.x == new_chunk_x && chunk.y == new_chunk_y {
                        list.entities.push(automaton);
                    }
                }
            }
            auto_position.x = new_position.x;
            auto_position.y = new_position.y;
        }
    }
}

pub fn bomb(
    mut commands: Commands,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    mut bomb_query: Query<(Entity, &mut component::Position, &mut component::Speed, &mut component::Velocity, &mut component::DeathTimer, &mut component::Bomb)>,
    spread_query: Query<&component::Position, (With<component::Spread>, Without<component::Bomb>, Without<component::Input>)>,
    land_query: Query<&component::Position, (With<component::Land>, Without<component::Spread>, Without<component::Bomb>, Without<component::Indestructable>, Without<component::Floor>)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>
    ) {
    for (entity, mut bomb_position, mut bomb_speed, mut bomb_velocity, mut bomb_timer, bomb_radius) in bomb_query.iter_mut() {
        let chunk_radius = ((bomb_radius.radius/crate::CHUNK_SIZE as f32).ceil() * 2.0) as i32;

        if bomb_speed.value >= 0.0 {
            bomb_speed.value -= time.delta_seconds() * 5.0;
        } else {
            bomb_speed.value  = 0.0;
        }

        let mut new_position = Vec2::new(bomb_position.x + (bomb_velocity.x * bomb_speed.value * time.delta_seconds()), bomb_position.y + (bomb_velocity.y * bomb_speed.value * time.delta_seconds()));
        let mut collide = false;
        let mut update_position = true;

        if new_position.x > (crate::WIDTH / crate::SCALE) as f32 {
            new_position.x = (crate::WIDTH / crate::SCALE)as f32 - 1.0;
            collide = true;
        }
        if new_position.x < 0.0 {
            new_position.x = 0.0;
            collide = true;
        }
        if new_position.y > ((crate::HEIGHT / crate::SCALE) - crate::MENU_SIZE) as f32 {
            new_position.y = ((crate::HEIGHT / crate::SCALE) - crate::MENU_SIZE) as f32 - 1.0;
            collide = true;
        }
        if new_position.y < 0.0 {
            new_position.y = 0.0;
            collide = true;
        }

        for (list, chunk) in chunk_query.iter_mut() {
            if chunk.x - (new_position.x as i32 / crate::CHUNK_SIZE) > -1 && chunk.x - (new_position.x as i32 / crate::CHUNK_SIZE) < 1 {
                if chunk.y - (new_position.y as i32 / crate::CHUNK_SIZE) > -1 && chunk.y - (new_position.y as i32 / crate::CHUNK_SIZE) < 1{
                    for list_entity in list.entities.iter() {
                        if let Ok(found_entity) = land_query.get(*list_entity) {
                            if found_entity.x as i32 == new_position.x as i32 && found_entity.y as i32 == new_position.y as i32 {
                                collide = true;
                                update_position = false;
                                bomb_velocity.x = 0.0;
                                bomb_velocity.y = 0.0;
                            }
                        }
                    }
                }
            }
        }

        if collide {
            bomb_timer.remaining = 0.0;
        }

        if update_position {
            bomb_position.x = new_position.x;
            bomb_position.y = new_position.y;
        }

        bomb_timer.remaining -= time.delta_seconds();
        if bomb_timer.remaining <= 0.0 {
            commands.entity(entity).despawn();
            commands.spawn(
                AudioBundle{
                    source: asset_server.load("./bomb.ogg"),
                    settings: PlaybackSettings{
                        mode: PlaybackMode::Despawn,
                        ..default()
                    },
                    ..default()
                });
            commands.spawn(bundle::FinderBundle {
                position: component::Position{ x: bomb_position.x, y: bomb_position.y },
                colour: component::Colour { r: utils::COLOUR_BEAM[0], g: utils::COLOUR_BEAM[1], b: utils::COLOUR_BEAM[2], a: utils::COLOUR_BEAM[3] },
                circle: component::Circle { radius: bomb_radius.radius },
                finder: component::Finder { minimum_radius: 0, maximum_radius: bomb_radius.radius as i32, timer_target: 0.25 , timer_counter: 0.0 }
            });
            for (list, chunk) in chunk_query.iter_mut() {
                if chunk.x - (bomb_position.x as i32 / crate::CHUNK_SIZE) > -chunk_radius && chunk.x - (bomb_position.x as i32 / crate::CHUNK_SIZE) < chunk_radius {
                    if chunk.y - (bomb_position.y as i32 / crate::CHUNK_SIZE) > -chunk_radius && chunk.y - (bomb_position.y as i32 / crate::CHUNK_SIZE) < chunk_radius {
                        for list_entity in list.entities.iter() {
                            if let Ok(found_entity) = spread_query.get(*list_entity) {
                                let distance = ((found_entity.x - bomb_position.x).powi(2) + (found_entity.y - bomb_position.y).powi(2)).sqrt();
                                if distance <= bomb_radius.radius { 
                                    commands.entity(*list_entity).despawn();
                                }
                            }
                            if let Ok(found_entity) = land_query.get(*list_entity) {
                                let distance = ((found_entity.x - bomb_position.x).powi(2) + (found_entity.y - bomb_position.y).powi(2)).sqrt();
                                if distance <= bomb_radius.radius { 
                                    commands.entity(*list_entity).despawn();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
