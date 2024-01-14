use bevy::prelude::*;
use rand::Rng;

use crate::bundle;
use crate::component;

pub fn spawn(
    mut commands: Commands,
    ) {
    commands.spawn(bundle::PixelBundle {
        position: component::Position { x: 175, y: 200 },
        colour: component::Colour { r: 91, g: 51, b: 115, a: 255 }
    }).insert(component::Spread {
        duration: 5.0,
        counter: 0.0
    }).insert(component::MoveTimer {
        duration: 0.2,
        counter: 0.0
    });
}

pub fn movement(
    mut commands: Commands,
    mut spread_query: Query<(&mut component::Position, &mut component::Spread, &mut component::MoveTimer), With<component::Spread>>,
    land_query: Query<(Entity, &component::Position), (With<component::Land>, Without<component::Spread>)>,
    time: Res<Time>
    ) {
    let mut rng = rand::thread_rng();
    for (mut position, mut spread, mut move_timer) in spread_query.iter_mut() {
        move_timer.counter += time.delta_seconds();

        if move_timer.counter >= move_timer.duration {
            move_timer.counter = 0.0;

            let mut new_position = IVec2::new(position.x, position.y);
            let direction = rng.gen_range(0..4);
            match direction {
                0 => new_position.y = i32::min(new_position.y + 1, crate::HEIGHT),
                1 => new_position.x = i32::min(new_position.x + 1, crate::WIDTH),
                2 => new_position.y = i32::max(new_position.y - 1, 0),
                3 => new_position.x = i32::max(new_position.x - 1, 0),
                _ => continue,
            }
            let mut update_position = true;

            for (land_entity, land_position) in land_query.iter() {
                if land_position.x == new_position.x && land_position.y == new_position.y {
                    update_position = false;
                    //commands.entity(land_entity).despawn();
                    //spread.counter += 1.0;
                    break;
                }
            }
            if update_position {
                position.x = new_position.x;
                position.y = new_position.y;
            }
        }
    }
}

pub fn spread(
    mut commands: Commands,
    mut spread_query: Query<(&component::Position, &component::Colour, &mut component::Spread)>,
    time: Res<Time>
    ) {
    let mut rng = rand::thread_rng();
    for (position, spread_colour, mut spread) in spread_query.iter_mut() {
        spread.counter += time.delta_seconds();

        if spread.counter >= spread.duration {
            spread.counter = 0.0;
            let direction = rng.gen_range(0..4);
            let mut new_position = IVec2::new(position.x, position.y);
            match direction {
                0 => new_position.y += 1,
                1 => new_position.x += 1,
                2 => new_position.y -= 1,
                3 => new_position.x -= 1,
                _ => new_position.y += 1
            }

            commands.spawn(bundle::PixelBundle {
                position: component::Position { x: new_position.x, y: new_position.y },
                colour: component::Colour { r: spread_colour.r, g: spread_colour.g, b: spread_colour.b, a: spread_colour.a }
            }).insert(component::Spread {
                duration: rng.gen_range(5.0..10.0),
                counter: 0.0
            }).insert(component::MoveTimer {
                duration: rng.gen_range(0.1..0.5),
                counter: 0.0
            });
        }
    }
}
