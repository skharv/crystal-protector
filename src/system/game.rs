use bevy::prelude::*;

use crate::bundle;
use crate::component;

pub fn setup(
    mut commands: Commands
    ) {
    for x in 0..((crate::WIDTH / crate::SCALE) / crate::CHUNK_SIZE) {
        for y in 0..(((crate::HEIGHT / crate::SCALE) - crate::MENU_SIZE) / crate::CHUNK_SIZE) {
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

pub fn death_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut component::DeathTimer), Without<component::Bomb>>,
    time: Res<Time>
    ) {
    for (entity, mut timer) in query.iter_mut() {
        timer.remaining -= time.delta_seconds();
        if timer.remaining <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_finder(
    mut commands: Commands,
    mut query: Query<(Entity, &mut component::Finder, &mut component::Circle)>,
    time: Res<Time>
    ) {
    for (entity, mut finder, mut circle) in query.iter_mut() {
        finder.timer_counter += time.delta_seconds();
        let percent = (1.0-(finder.timer_counter / finder.timer_target)) * (finder.maximum_radius - finder.minimum_radius) as f32;
        circle.radius = finder.minimum_radius as f32 + percent;
        if finder.timer_counter >= finder.timer_target {
            commands.entity(entity).despawn();
        }
    }
}
