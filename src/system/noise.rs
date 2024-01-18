use bevy::prelude::*;
use noise::{Fbm, RidgedMulti, NoiseFn, Perlin};
use rand::Rng;

use crate::component;
use crate::bundle;

const DEEP: [u8; 4] = [32, 17, 39, 255];
const MEDIUM: [u8; 4] = [32, 20, 51, 255];
const SHALLOW: [u8; 4] = [27, 30, 52, 255];
const RESOURCE: [u8; 4] = [255, 255, 255, 255];

pub fn generate(
    mut commands: Commands,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>,
    ) {
    let mut rng = rand::thread_rng();
    let mut fbm = Fbm::<Perlin>::new(rng.gen_range(0..10000));
    fbm.octaves = rng.gen_range(2..5);
    fbm.frequency = rng.gen_range(0.02..0.04);


    for w in 0..(crate::WIDTH/crate::SCALE) {
        for h in 0..(crate::HEIGHT/crate::SCALE) {
            let noise = fbm.get([w as f64, h as f64]);
            let colour;

            if noise < 0.0 {
                continue;
            } else if noise < 0.3 {
                colour = SHALLOW;
            } else if noise < 0.6 {
                colour = MEDIUM;
            } else {
                colour = DEEP;
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

            for (mut list, position) in chunk_query.iter_mut() {
                if (w / crate::CHUNK_SIZE) == position.x && (h / crate::CHUNK_SIZE) == position.y {
                    list.entities.push(id);
                }
            }
        }
    }
}

pub fn generate_resources (
    mut commands: Commands,
    mut land_query: Query<(Entity, &component::Position, &mut component::Colour), With<component::Land>>
    ) {
    let mut rng = rand::thread_rng();
    let mut rm = RidgedMulti::<Perlin>::new(rng.gen_range(0..10000));
    rm.octaves = 5;
    rm.frequency = rng.gen_range(0.02..0.04);


    for w in 0..(crate::WIDTH/crate::SCALE) {
        for h in 0..(crate::HEIGHT/crate::SCALE) {
            let noise = rm.get([w as f64, h as f64]);

            if noise < 0.5 {
                continue;
            }

            for (entity, position, mut colour) in land_query.iter_mut() {
                if w == position.x as i32 && h == position.y as i32 {
                    colour.r = RESOURCE[0];
                    colour.g = RESOURCE[1];
                    colour.b = RESOURCE[2];
                    colour.a = RESOURCE[3];

                    commands.entity(entity).insert(component::Resource);
                }
            }
        }
    }
}
