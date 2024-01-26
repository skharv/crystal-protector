use bevy::prelude::*;

use crate::component;

pub fn timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut component::Timer), (With<component::Beam>, Without<component::Dying>)>,
    time: Res<Time>
    ) {
    for (entity, mut timer) in query.iter_mut() {
        timer.remaining -= time.delta_seconds();
        if timer.remaining <= 0.0 {
            commands.entity(entity).insert(component::Dying);
        }
    }
}
