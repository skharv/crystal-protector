use bevy::prelude::*;
use bevy_pixels::*;

use crate::component;
use crate::utils;

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
    query: Query<(&component::Position, &component::Colour), Without<component::Ui>>,
    symbol_query: Query<(&component::Position, &component::Colour, &component::Symbol), With<component::Ui>>,
    bar_query: Query<(&component::Position, &component::Colour, &component::Bar), With<component::Ui>>
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

    for (position, color, symbol) in symbol_query.iter() {
        let bool_list = utils::convert_string_to_symbol(&symbol.shape);
        
        for i in 0..bool_list.len() {
            if bool_list[i] {
            let index = (((position.y as i32 + (i / crate::SYMBOL_SIZE) as i32) * 4 * (crate::WIDTH/crate::SCALE)) + (position.x as i32 + (i % crate::SYMBOL_SIZE) as i32) * 4) as usize;

            if index < frame.iter().count() {
                frame[index] = color.r;
                frame[index+1] = color.g;
                frame[index+2] = color.b;
                frame[index+3] = color.a;
            }
            }
        }
    }

    for (position, color, bar) in bar_query.iter() {
        
        for w in 0..bar.width {
        for h in 0..bar.height {
            if w == 0 || h == 0 || w == bar.width-1 || h == bar.height-1 || ((w as f32/bar.width as f32)*100.0 <= bar.percent) {
            let index = (((position.y as i32 + h) * 4 * (crate::WIDTH/crate::SCALE)) + (position.x as i32 + w) * 4) as usize;

            if index < frame.iter().count() {
                frame[index] = color.r;
                frame[index+1] = color.g;
                frame[index+2] = color.b;
                frame[index+3] = color.a;
            }
            }
        }
        }
    }
}
