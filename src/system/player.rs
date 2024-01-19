use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::utils;
use crate::bundle;
use crate::component;

pub fn spawn(
    mut commands: Commands,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    land_query: Query<&component::Position, (With<component::Land>, Without<component::Spread>)>,
    ) {
    let mut rng = rand::thread_rng();

    let mut x = rng.gen_range(0..crate::WIDTH/crate::SCALE);
    let mut y = rng.gen_range(0..crate::HEIGHT/crate::SCALE);

    while land_query.iter().any(|pos| pos.x as i32 == x as i32 && pos.y as i32 == y as i32) {
        x = rng.gen_range(0..crate::WIDTH/crate::SCALE);
        y = rng.gen_range(0..crate::HEIGHT/crate::SCALE);
    }

    let entity = commands.spawn(bundle::PlayerBundle {
        position: component::Position { x: x as f32, y: y as f32 },
        velocity: component::Velocity { x: 0.0, y: 0.0 },
        speed: component::Speed { value: 20.0 },
        colour: component::Colour { r: utils::COLOUR_PLAYER[0], g: utils::COLOUR_PLAYER[1], b: utils::COLOUR_PLAYER[2], a: utils::COLOUR_PLAYER[3] },
        input: component::Input { angle: None },
        absorb: component::Absorb { radius: 2.5, range: 20.0 },
        action: component::Action { action: utils::Action::House },
        resources: component::Resources { maximum: 1000, amount: 0 }
    }).id();

    commands.spawn(bundle::FinderBundle {
        position: component::Position { x: x as f32, y: y as f32 },
        colour: component::Colour { r: utils::COLOUR_PLAYER[0], g: utils::COLOUR_PLAYER[1], b: utils::COLOUR_PLAYER[2], a: utils::COLOUR_PLAYER[3] },
        circle: component::Circle { radius: 100.0 },
        finder: component::Finder { minimum_radius: 5, maximum_radius: 100, timer_target: 0.75, timer_counter: 0.0 },
    });

    for (mut list, chunk) in chunk_query.iter_mut() {
        if chunk.x == (x / crate::CHUNK_SIZE) && chunk.y == (y / crate::CHUNK_SIZE) {
            list.entities.push(entity);
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

pub fn update_velocity(
    mut query: Query<(&mut component::Velocity, &component::Input, &component::Speed)>
    ) {
    for (mut velocity, input, speed) in query.iter_mut() {
        if let Some(direction) = input.angle {
            velocity.x = f32::sin(direction) * -speed.value;
            velocity.y = f32::cos(direction) * -speed.value;
        } else {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

pub fn update_position(
    mut query: Query<(&mut component::Position, &component::Velocity), With<component::Input>>,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    land_query: Query<&component::Position, (With<component::Land>, Without<component::Input>, Without<component::Spread>)>,
    time: Res<Time>
    ) {
    for (mut position, velocity) in query.iter_mut() {
        let mut new_position = Vec2::new(position.x + (velocity.x * time.delta_seconds()), position.y + (velocity.y * time.delta_seconds()));

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

        let mut update_position = true;

        for (list, chunk) in chunk_query.iter_mut() {
            if chunk.x - (new_position.x as i32 / crate::CHUNK_SIZE) > -1 && chunk.x - (new_position.x as i32 / crate::CHUNK_SIZE) < 1 {
                if chunk.y - (new_position.y as i32 / crate::CHUNK_SIZE) > -1 && chunk.y - (new_position.y as i32 / crate::CHUNK_SIZE) < 1{
                    for list_entity in list.entities.iter() {
                        if let Ok(found_entity) = land_query.get(*list_entity) {
                            if found_entity.x as i32 == new_position.x as i32 && found_entity.y as i32 == new_position.y as i32 {
                                update_position = false;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if update_position {
            position.x = new_position.x;
            position.y = new_position.y;
        }
    }
}


pub fn update_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut component::Input>
    ) {
    for mut input in query.iter_mut() {
        let mut vector = Vec2::new(0.0, 0.0);
        let mut be_none = true;
        if keys.pressed(KeyCode::W) {
            vector.y += 1.0;
            be_none = false;
        }
        if keys.pressed(KeyCode::A) {
            vector.x += 1.0;
            be_none = false;
        }
        if keys.pressed(KeyCode::S) {
            vector.y -= 1.0;
            be_none = false;
        }
        if keys.pressed(KeyCode::D) {
            vector.x -= 1.0;
            be_none = false;
        }
        vector = vector.normalize();
        if be_none {
            input.angle = None;
        } else {
            input.angle = Some(vector.x.atan2(vector.y));
        }
    }
}

pub fn absorb(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut absorb_query: Query<(&component::Position, &mut component::Resources, &component::Absorb), With<component::Input>>,
    mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    mut land_query: Query<(&component::Position, &mut component::Colour, Option<&component::Resource>), (With<component::Land>, Without<component::Input>, Without<component::Spread>, Without<component::DeathTimer>)>
    ) {
    let mut rng = rand::thread_rng();
    if buttons.just_pressed(MouseButton::Right) {
        if let Some(cursor_position) = window.single().cursor_position() {
            for (position, mut resources, absorb) in absorb_query.iter_mut() { 
                let distance = ((cursor_position.x/2.0 - position.x).powi(2) + (cursor_position.y/2.0 - position.y).powi(2)).sqrt();
                for step in 1..absorb.range as i32 {
                    let x = position.x + (((step as f32) * (cursor_position.x/2.0 - position.x)) / distance);
                    let y = position.y + (((step as f32) * (cursor_position.y/2.0 - position.y)) / distance);
                    commands.spawn(bundle::BeamBundle {
                        position: component::Position{ x, y },
                        colour: component::Colour{ r: utils::COLOUR_BEAM[0], g: utils::COLOUR_BEAM[1], b: utils::COLOUR_BEAM[2], a: utils::COLOUR_BEAM[3]},
                        timer: component::Timer{ remaining: rng.gen_range(0.05..0.25) },
                        beam: component::Beam
                    });

                    let mut target_position = Vec2::new(0.0, 0.0);
                    let mut target_chunk = IVec2::new(0, 0);
                    let mut target_hit = false;

                    'finder: for (list, chunk) in chunk_query.iter_mut() {
                        if chunk.x - (x as i32 / crate::CHUNK_SIZE) > -1 && chunk.x - (x as i32 / crate::CHUNK_SIZE) < 1 {
                            if chunk.y - (y as i32 / crate::CHUNK_SIZE) > -1 && chunk.y - (y as i32 / crate::CHUNK_SIZE) < 1{
                                for list_entity in list.entities.iter() {
                                    if let Ok((found_entity, _, _)) = land_query.get_mut(*list_entity) {
                                        if found_entity.x as i32 == x as i32 && found_entity.y as i32 == y as i32 {
                                            target_hit = true;
                                            target_position = Vec2::new(found_entity.x, found_entity.y);
                                            target_chunk = IVec2::new(chunk.x, chunk.y);
                                            break 'finder;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if target_hit {
                        for (list, chunk) in chunk_query.iter_mut() {
                            if chunk.x - target_chunk.x > -2 && chunk.x - target_chunk.x < 2 {
                                if chunk.y - target_chunk.y > -2 && chunk.y - target_chunk.y < 2{
                                    for list_entity in list.entities.iter() {
                                        if let Ok((found_entity, mut found_colour, found_resource)) = land_query.get_mut(*list_entity) {
                                            let distance = ((found_entity.x - target_position.x).powi(2) + (found_entity.y - target_position.y).powi(2)).sqrt();
                                            if distance <= absorb.radius {
                                                found_colour.r = utils::COLOUR_BEAM[0];
                                                found_colour.g = utils::COLOUR_BEAM[1];
                                                found_colour.b = utils::COLOUR_BEAM[2];
                                                found_colour.a = utils::COLOUR_BEAM[3];
                                                commands.entity(*list_entity).insert(component::DeathTimer{ remaining: rng.gen_range(0.05..0.25) });
                                                if let Some(resource) = found_resource {
                                                    if resources.amount < resources.maximum {
                                                        resources.amount = (resources.amount + resource.value).min(resources.maximum);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        return;
                    }
                }
            }
        }
    }
}

pub fn swap_action(
    keys: Res<Input<KeyCode>>,
    mut action_query: Query<&mut component::Action, With<component::Input>>,
    ) {
    if keys.just_pressed(KeyCode::Tab) {
        for mut action in action_query.iter_mut() {
            match action.action {
                utils::Action::House => action.action = utils::Action::Bomb,
                utils::Action::Bomb => action.action = utils::Action::Face,
                utils::Action::Face => action.action = utils::Action::Factory,
                utils::Action::Factory => action.action = utils::Action::Bubble,
                utils::Action::Bubble => action.action = utils::Action::House,
            }
        }
    }
    if keys.just_pressed(KeyCode::Key1) {
        for mut action in action_query.iter_mut() {
            action.action = utils::Action::House;
        }
    }
    if keys.just_pressed(KeyCode::Key2) {
        for mut action in action_query.iter_mut() {
            action.action = utils::Action::Bomb;
        }
    }
    if keys.just_pressed(KeyCode::Key3) {
        for mut action in action_query.iter_mut() {
            action.action = utils::Action::Face;
        }
    }
    if keys.just_pressed(KeyCode::Key4) {
        for mut action in action_query.iter_mut() {
            action.action = utils::Action::Factory;
        }
    }
    if keys.just_pressed(KeyCode::Key5) {
        for mut action in action_query.iter_mut() {
            action.action = utils::Action::Bubble;
        }
    }
}

pub fn action(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut action_query: Query<(&component::Position, &mut component::Resources, &component::Action), With<component::Input>>,
    //mut chunk_query: Query<(&mut component::EntityList, &component::Chunk)>, 
    //mut land_query: Query<(&component::Position, &mut component::Colour), (With<component::Land>, Without<component::Input>, Without<component::Spread>)>
    ) {
    let mut rng = rand::thread_rng();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(_) = window.single().cursor_position() {
            for (position, mut resources, action) in action_query.iter_mut() { 
                match action.action {
                    utils::Action::Bomb => {
                    },
                    utils::Action::Face => {
                        if resources.amount >= 80 {
                            let angle = rng.gen_range(0.0..360.0);
                            let vel = Vec2::new(f32::sin(angle), f32::cos(angle));
                            commands.spawn(bundle::AutomatonBundle {
                                position: component::Position{ x: position.x, y: position.y },
                                velocity: component::Velocity { x: vel.x, y: vel.y },
                                speed: component::Speed { value: 20.0 },
                                colour: component::Colour { r: utils::COLOUR_BEAM[0], g: utils::COLOUR_BEAM[1], b: utils::COLOUR_BEAM[2], a: utils::COLOUR_BEAM[3] },
                                timer: component::DeathTimer { remaining: 30.0 },
                                automaton: component::Automaton
                            });
                            resources.amount -= 80;
                        }
                    },
                    utils::Action::Factory => {
                    },
                    utils::Action::Bubble => {
                        if resources.amount >= 50 {
                            commands.spawn(bundle::BubbleBundle {
                                position: component::Position{ x: position.x, y: position.y },
                                circle: component::Circle { radius: 10.0 },
                                timer: component::DeathTimer { remaining: 60.0 },
                                colour: component::Colour { r: utils::COLOUR_SPREAD[0], g: utils::COLOUR_SPREAD[1], b: utils::COLOUR_SPREAD[2], a: utils::COLOUR_SPREAD[3] },
                                bubble: component::Bubble
                            });

                            resources.amount -= 50;
                        }
                    },
                    utils::Action::House => {
                    },
                }
            }
        }
    }
}

