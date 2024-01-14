use bevy::prelude::*;
use bevy_pixels::*;

use crate::component;

pub fn clear(
    mut wrapper_query: Query<&mut PixelsWrapper>
    ) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    frame.copy_from_slice(&[199, 115, 105, 255].repeat(frame.len()/4));
}

pub fn draw(
    mut wrapper_query: Query<&mut PixelsWrapper>,
    query: Query<(&component::Position, &component::Colour)>
    ) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();

    for (position, color) in query.iter() {
        let index = ((position.y * 4 * (crate::WIDTH/crate::SCALE)) + position.x * 4) as usize;

        if index < frame.iter().count() {
            frame[index] = color.r;
            frame[index+1] = color.g;
            frame[index+2] = color.b;
            frame[index+3] = color.a;
        }
    }
}
