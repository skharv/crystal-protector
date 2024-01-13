use bevy::prelude::*;
use noise::{Fbm, NoiseFn, Perlin};

use crate::component;
use crate::bundle;

pub fn generate(
    mut commands: Commands
    ) {
    let mut fbm = Fbm::<Perlin>::new(0);
    fbm.octaves = 4;
    fbm.frequency = 0.005;

    for w in 0..crate::WIDTH {
        for h in 0..crate::HEIGHT {
            let mut noise = fbm.get([w as f64, h as f64]);
            if noise < 0.0 {
                continue;
            } else {
                noise = 1.0;
            }
            let colour_value = (((noise + 1.0) * 0.5) * 255.0) as u8;
            commands.spawn(bundle::PixelBundle {
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
            });
        }
    }
}
