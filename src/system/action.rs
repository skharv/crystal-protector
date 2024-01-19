use bevy::prelude::*;

use crate::component;

pub fn bubble (
    bubble_query: Query<(&component::Position, &component::Circle), With<component::Bubble>>,
    chunk_query: Query<(&component::EntityList, &component::Chunk)>, 
    mut spread_query: Query<(&component::Position, &mut component::Velocity, &mut component::Hunger), With<component::Spread>>,
    time: Res<Time>
    ) {
    for (bubble_position, bubble_circle) in bubble_query.iter() {
        let chunk_radius = (bubble_circle.radius as f32 / crate::CHUNK_SIZE as f32).ceil() as i32;
        for (chunk_list, chunk_pos) in chunk_query.iter() {
            if chunk_pos.x - (bubble_position.x as i32 / crate::CHUNK_SIZE) > -chunk_radius && chunk_pos.x - (bubble_position.x as i32 / crate::CHUNK_SIZE) < chunk_radius {
                if chunk_pos.y - (bubble_position.y as i32 / crate::CHUNK_SIZE) > -chunk_radius && chunk_pos.y - (bubble_position.y as i32 / crate::CHUNK_SIZE) < chunk_radius {
                    for list_entity in chunk_list.entities.iter() {
                        if let Ok((spread_position, mut spread_velocity, mut spread_hunger)) = spread_query.get_mut(*list_entity) {
                            let x = bubble_position.x - spread_position.x;
                            let y = bubble_position.y - spread_position.y;
                            let distance = (x.powi(2) + y.powi(2)).sqrt();
                            if distance <= bubble_circle.radius {
                                let angle = y.atan2(x);
                                spread_velocity.x = angle.sin();
                                spread_velocity.y = angle.cos();
                                spread_hunger.duration -= 5.0 * time.delta_seconds();
                            }
                        }
                    }
                }
            }
        }
    }
}
