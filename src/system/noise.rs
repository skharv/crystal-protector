use bevy::prelude::*;
use noise::{Fbm, RidgedMulti, NoiseFn, Perlin};
use rand::Rng;

use crate::component;
use crate::bundle;
use crate::utils;

pub fn generate(
    mut commands: Commands,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>,
    ) {
    let mut rng = rand::thread_rng();
    let seed = rng.gen_range(0..10000);
    let mut fbm = Fbm::<Perlin>::new(seed);
    fbm.octaves = rng.gen_range(2..5);
    fbm.frequency = rng.gen_range(0.02..0.04);

    let mut rm = RidgedMulti::<Perlin>::new(seed);
    rm.octaves = 5;
    rm.frequency = rng.gen_range(0.02..0.04);

    for w in 0..(crate::WIDTH/crate::SCALE) {
        for h in 0..(crate::HEIGHT/crate::SCALE) - crate::MENU_SIZE {
            let noise = fbm.get([w as f64, h as f64]);
            let resource_noise = rm.get([w as f64, h as f64]);
            let mut colour;
            let floor_colour;

            if noise > -0.3 {
                floor_colour = utils::COLOUR_DARK_FLOOR;
            } else {
                floor_colour = utils::COLOUR_LIGHT_FLOOR;
            }

            commands.spawn(bundle::FloorBundle {
                position: component::Position{
                    x: w as f32,
                    y: h as f32
                },
                colour: component::Colour{
                    r: floor_colour[0],
                    g: floor_colour[1],
                    b: floor_colour[2],
                    a: floor_colour[3]
                },
                floor: component::Floor
            });

            if noise < 0.0 {
                continue;
            } else if noise < 0.3 {
                colour = utils::COLOUR_SHALLOW;
            } else if noise < 0.6 {
                colour = utils::COLOUR_MEDIUM;
            } else {
                colour = utils::COLOUR_DEEP;
            }

            
            if resource_noise > 0.6 {
                colour = utils::COLOUR_RICH_RESOURCE;
            } else if resource_noise > 0.4 {
                colour = utils::COLOUR_RESOURCE;
            }
                
            let id = commands.spawn(bundle::LandBundle {
                position: component::Position{
                    x: w as f32,
                    y: h as f32
                },
                colour: component::Colour{
                    r: colour[0],
                    g: colour[1],
                    b: colour[2],
                    a: colour[3]
                },
                land: component::Land
            })
            .id();

            if resource_noise > 0.6 {
                commands.entity(id).insert(component::Resource{value: 5});
            } else if resource_noise > 0.4 {
                commands.entity(id).insert(component::Resource{value: 1});
            }

            for (mut list, position) in chunk_query.iter_mut() {
                if (w / crate::CHUNK_SIZE) == position.x && (h / crate::CHUNK_SIZE) == position.y {
                    list.entities.push(id);
                }
            }
        }
    }
}
