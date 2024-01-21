use bevy::prelude::*;

use crate::component;
use crate::bundle;
use crate::utils;

pub fn spawn(
    mut commands: Commands,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    land_query: Query<&component::Position, (With<component::Land>, Without<component::Spread>)>,
    ) {
    let start_x = (((crate::WIDTH / crate::SCALE) as f32 / 2.0) - crate::SYMBOL_SIZE as f32 / 2.0) as i32;
    let start_y = (((crate::HEIGHT / crate::SCALE) as f32 / 2.0) - crate::SYMBOL_SIZE as f32 / 2.0) as i32;

    let bool_list = utils::convert_string_to_symbol(&utils::SYMBOL_BASE_SHAPE.to_string());
    let mut spawn_counter = 0;

    for x in start_x..start_x+crate::SYMBOL_SIZE as i32 {
        for y in start_y..start_y+crate::SYMBOL_SIZE as i32 {
            for (mut list, chunk) in chunk_query.iter_mut() {
                if chunk.x == (x / crate::CHUNK_SIZE) && chunk.y == (y / crate::CHUNK_SIZE) {
                    let mut id = None;
                    let mut spawn = false;
                    if list.entities.iter().len() <= spawn_counter {
                        if utils::is_position_part_of_symbol(x - start_x, y - start_y, bool_list) {
                            id = Some(commands.spawn(bundle::BaseSectionBundle {
                                position: component::Position { x: x as f32, y: y as f32 },
                                colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
                                land: component::Land,
                                indestructable: component::Indestructable
                            }).id());
                            spawn = true;
                        }
                    } else {
                        for list_entity in list.entities.iter() {
                            if let Ok(found_entity) = land_query.get(*list_entity) {
                                if found_entity.x as i32 == x && found_entity.y as i32 == y {
                                    commands.entity(*list_entity).despawn();
                                }
                                if utils::is_position_part_of_symbol(x - start_x, y - start_y, bool_list) {
                                    id = Some(commands.spawn(bundle::BaseSectionBundle {
                                        position: component::Position { x: x as f32, y: y as f32 },
                                        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
                                        land: component::Land,
                                        indestructable: component::Indestructable
                                    }).id());
                                    spawn = true;
                                }
                            }
                        }
                    }
                    if spawn {
                        if let Some(entity) = id {
                            list.entities.push(entity);
                            spawn_counter = spawn_counter + 1;
                        }
                    }
                }
            }
        }
    }
}
