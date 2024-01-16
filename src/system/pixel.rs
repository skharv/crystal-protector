use bevy::prelude::*;
use bevy_pixels::*;

use crate::component;

const CLEAR: [u8; 4] = [53, 93, 104, 255];

pub fn clear(
    mut wrapper_query: Query<&mut PixelsWrapper>
    ) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    frame.copy_from_slice(&CLEAR.repeat(frame.len()/4));
}

pub fn draw(
    mut wrapper_query: Query<&mut PixelsWrapper>,
    query: Query<(&component::Position, &component::Colour)>,
    ) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();

    for (position, color) in query.iter() {
        let mut new_position = Vec2::new(position.x, position.y);
        if new_position.x > (crate::WIDTH / crate::SCALE) as f32 {
            new_position.x = (crate::WIDTH / crate::SCALE)as f32;
        }
        if new_position.x < 0.0 {
            new_position.x = 0.0;
        }
        if new_position.y > (crate::HEIGHT / crate::SCALE) as f32 {
            new_position.y = (crate::HEIGHT / crate::SCALE) as f32;
        }
        if new_position.y < 0.0 {
            new_position.y = 0.0;
        }
        let index = ((new_position.y as i32 * 4 * (crate::WIDTH/crate::SCALE)) + new_position.x as i32 * 4) as usize;

        if index < frame.iter().count() {
            frame[index] = color.r;
            frame[index+1] = color.g;
            frame[index+2] = color.b;
            frame[index+3] = color.a;
        }
    }
}
