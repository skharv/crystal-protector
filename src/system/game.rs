use bevy::prelude::*;

use crate::bundle;
use crate::component;

pub fn setup(
    mut commands: Commands
    ) {
    for x in 0..((crate::WIDTH / crate::SCALE) / crate::CHUNK_SIZE) {
        for y in 0..((crate::HEIGHT / crate::SCALE) / crate::CHUNK_SIZE) {
            commands.spawn(bundle::ChunkBundle {
                position: component::Chunk {
                    x,
                    y
                },
                entities: component::EntityList{
                    entities: Vec::new()
                }
            });
        }
    }
}
