use bevy::prelude::*;
use noise::{Fbm, NoiseFn, Perlin};

use crate::component;
use crate::bundle;

pub fn generate(
    mut commands: Commands,
    mut chunk_query: Query<(&mut component::ChunkList, &component::Position)>
    ) {
    let mut fbm = Fbm::<Perlin>::new(0);
    fbm.octaves = 4;
    fbm.frequency = 0.02;

    for w in 0..(crate::WIDTH/crate::SCALE) {
        for h in 0..(crate::HEIGHT/crate::SCALE) {
            let mut noise = fbm.get([w as f64, h as f64]);
            if noise < 0.0 {
                continue;
            } else {
                noise = 0.5;
            }
            let colour_value = (((noise + 1.0) * 0.5) * 255.0) as u8;
            let id = commands.spawn(bundle::PixelBundle {
                position: component::Position{
                    x: w,
                    y: h
                },
                colour: component::Colour{
                    r: colour_value,
                    g: colour_value,
                    b: colour_value,
                    a: colour_value
                }
            }).insert(component::Land)
            .id();

            for (mut list, position) in chunk_query.iter_mut() {
                if (w / crate::CHUNK_SIZE) == position.x && (h / crate::CHUNK_SIZE) == position.y {
                    list.entities.push(id);
                }
            }
        }
    }
}
