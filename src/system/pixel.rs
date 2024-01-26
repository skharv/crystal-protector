use bevy::prelude::*;
use bevy_pixels::*;
use rand::Rng;

use crate::component;
use crate::utils;

pub fn clear(
    mut wrapper_query: Query<&mut PixelsWrapper>
    ) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    frame.copy_from_slice(&utils::COLOUR_DEEP.repeat(frame.len()/4));
}

pub fn draw(
    mut wrapper_query: Query<&mut PixelsWrapper>,
    floor_query: Query<(&component::Position, &component::Colour), With<component::Floor>>,
    query: Query<(&component::Position, &component::Colour), (Without<component::Ui>, Without<component::Circle>)>,
    symbol_query: Query<(&component::Position, &component::Colour, &component::Symbol), (With<component::Ui>, Without<component::Victory>, Without<component::Defeat>, Without<component::Start>)>,
    bar_query: Query<(&component::Position, &component::Colour, &component::Size, Option<&component::Bar>), With<component::Ui>>,
    circle_query: Query<(&component::Position, &component::Colour, &component::Circle, Option<&component::Bubble>)>,
    ) {
    let mut rng = rand::thread_rng();

    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    for (position, colour) in floor_query.iter() {
        let index = ((position.y as i32 * 4 * (crate::WIDTH/crate::SCALE)) + position.x as i32 * 4) as usize;

        if index < frame.iter().count() {
            frame[index] = colour.r;
            frame[index+1] = colour.g;
            frame[index+2] = colour.b;
            frame[index+3] = colour.a;
        }
    }

    for (position, colour) in query.iter() {
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
            frame[index] = colour.r;
            frame[index+1] = colour.g;
            frame[index+2] = colour.b;
            frame[index+3] = colour.a;
        }
    }

    for (position, colour, circle, bubble) in circle_query.iter() {
        for t in 0..360 {
        let mut new_position = Vec2::new(position.x  + (circle.radius as f32 * f32::sin(t as f32)), position.y + (circle.radius as f32 * f32::cos(t as f32)));

        if new_position.x > (crate::WIDTH / crate::SCALE) as f32 {
            new_position.x = (crate::WIDTH / crate::SCALE) as f32;
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

        let index = ((new_position.y as i32 * 4 * (crate::WIDTH/crate::SCALE)) + (new_position.x as i32 * 4)) as usize;
        if index < frame.iter().count() {
            frame[index] = colour.r;
            frame[index+1] = colour.g;
            frame[index+2] = colour.b;
            if let Some(_) = bubble {
                frame[index+3] = rng.gen_range(0..255);
            } else {
                frame[index+3] = colour.a;
            }
        }
        }
    }

    for (position, colour, symbol) in symbol_query.iter() {
        let bool_list = utils::convert_string_to_symbol(&symbol.shape);
        
        for i in 0..bool_list.len() {
            if bool_list[i] {
            let index = (((position.y as i32 + (i / crate::SYMBOL_SIZE) as i32) * 4 * (crate::WIDTH/crate::SCALE)) + (position.x as i32 + (i % crate::SYMBOL_SIZE) as i32) * 4) as usize;

            if index < frame.iter().count() {
                frame[index] = colour.r;
                frame[index+1] = colour.g;
                frame[index+2] = colour.b;
                frame[index+3] = colour.a;
            }
            }
        }
    }

    for (position, colour, size, bar) in bar_query.iter() {
        for w in 0..size.width {
            for h in 0..size.height {
                if w == 0 || h == 0 || w == size.width-1 || h == size.height-1 {
                    let index = (((position.y as i32 + h) * 4 * (crate::WIDTH/crate::SCALE)) + (position.x as i32 + w) * 4) as usize;

                    if index < frame.iter().count() {
                        frame[index] = colour.r;
                        frame[index+1] = colour.g;
                        frame[index+2] = colour.b;
                        frame[index+3] = colour.a;
                    }
                } 
                if let Some(data) = bar {
                    if (w as f32/size.width as f32)*100.0 <= data.percent {
                        let index = (((position.y as i32 + h) * 4 * (crate::WIDTH/crate::SCALE)) + (position.x as i32 + w) * 4) as usize;
                        let mut new_colour: [u8; 4] = [colour.r, colour.g, colour.b, colour.a];

                        if (w as f32/size.width as f32)*100.0 >= (data.percent - data.cost) {
                            new_colour = utils::COLOUR_BEAM;
                        }

                        if index < frame.iter().count() {
                            frame[index] = new_colour[0];
                            frame[index+1] = new_colour[1];
                            frame[index+2] = new_colour[2];
                            frame[index+3] = new_colour[3];
                        }
                    }
                }
            }
        }
    }
}

pub fn draw_victory(
    mut wrapper_query: Query<&mut PixelsWrapper>,
    symbol_query: Query<(&component::Position, &component::Colour, &component::Symbol), (With<component::Ui>, With<component::Victory>)>,
    ) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    for (position, colour, symbol) in symbol_query.iter() {
        let bool_list = utils::convert_string_to_symbol(&symbol.shape);
        
        for i in 0..bool_list.len() {
            if bool_list[i] {
            let index = (((position.y as i32 + (i / crate::SYMBOL_SIZE) as i32) * 4 * (crate::WIDTH/crate::SCALE)) + (position.x as i32 + (i % crate::SYMBOL_SIZE) as i32) * 4) as usize;

            if index < frame.iter().count() {
                frame[index] = colour.r;
                frame[index+1] = colour.g;
                frame[index+2] = colour.b;
                frame[index+3] = colour.a;
            }
            }
        }
    }
}

pub fn draw_defeat(
    mut wrapper_query: Query<&mut PixelsWrapper>,
    symbol_query: Query<(&component::Position, &component::Colour, &component::Symbol), (With<component::Ui>, With<component::Defeat>)>,
    ) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    for (position, colour, symbol) in symbol_query.iter() {
        let bool_list = utils::convert_string_to_symbol(&symbol.shape);
        
        for i in 0..bool_list.len() {
            if bool_list[i] {
            let index = (((position.y as i32 + (i / crate::SYMBOL_SIZE) as i32) * 4 * (crate::WIDTH/crate::SCALE)) + (position.x as i32 + (i % crate::SYMBOL_SIZE) as i32) * 4) as usize;

            if index < frame.iter().count() {
                frame[index] = colour.r;
                frame[index+1] = colour.g;
                frame[index+2] = colour.b;
                frame[index+3] = colour.a;
            }
            }
        }
    }
}

pub fn draw_start(
    mut wrapper_query: Query<&mut PixelsWrapper>,
    symbol_query: Query<(&component::Position, &component::Colour, &component::Symbol), (With<component::Ui>, With<component::Start>)>,
    ) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();
    for (position, colour, symbol) in symbol_query.iter() {
        let bool_list = utils::convert_string_to_symbol(&symbol.shape);
        
        for i in 0..bool_list.len() {
            if bool_list[i] {
            let index = (((position.y as i32 + (i / crate::SYMBOL_SIZE) as i32) * 4 * (crate::WIDTH/crate::SCALE)) + (position.x as i32 + (i % crate::SYMBOL_SIZE) as i32) * 4) as usize;

            if index < frame.iter().count() {
                frame[index] = colour.r;
                frame[index+1] = colour.g;
                frame[index+2] = colour.b;
                frame[index+3] = colour.a;
            }
            }
        }
    }
}
