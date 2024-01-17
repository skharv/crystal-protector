use bevy::prelude::*;

use crate::bundle;
use crate::component;
use crate::utils;

const UI_COLOR: [u8; 4] = [255, 235, 153, 255];

pub fn generate_symbols(
    mut commands: Commands
    ) {
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: 10.0, y: 10.0 },
        colour: component::Colour { r: UI_COLOR[0], g: UI_COLOR[1], b: UI_COLOR[2], a: UI_COLOR[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_RESOURCE.into() },
        ui: component::Ui
    });

    commands.spawn(bundle::BarBundle {
        position: component::Position { x: 20.0, y: 10.0 },
        colour: component::Colour { r: UI_COLOR[0], g: UI_COLOR[1], b: UI_COLOR[2], a: UI_COLOR[3] },
        bar: component::Bar { width: 64, height: 8, percent: 25.0 },
        ui: component::Ui
    });
    
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: 10.0, y: 20.0 },
        colour: component::Colour { r: UI_COLOR[0], g: UI_COLOR[1], b: UI_COLOR[2], a: UI_COLOR[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_BULLET.into() },
        ui: component::Ui
    }).insert(component::Action);
}

pub fn update_bars(
    mut bar_query: Query<&mut component::Bar, With<component::Ui>>,
    absorb_query: Query<&component::Resources, With<component::Absorb>>
    ) {
    for resources in absorb_query.iter() {
        for mut bar in bar_query.iter_mut() {
            bar.percent = (resources.amount as f32 / resources.maximum as f32) * 100.0;
        }
    }
}

pub fn action(
    keys: Res<Input<KeyCode>>,
    mut symbol_query: Query<&mut component::Symbol, (With<component::Ui>, With<component::Action>)>
) {
    if keys.just_pressed(KeyCode::Tab) {
        for mut symbol in symbol_query.iter_mut() {
            match symbol.shape.as_str() {
               utils::SYMBOL_HOUSE => symbol.shape = utils::SYMBOL_BULLET.into(),
               utils::SYMBOL_BULLET => symbol.shape = utils::SYMBOL_BOMB.into(),
               utils::SYMBOL_BOMB => symbol.shape = utils::SYMBOL_FACE.into(),
               utils::SYMBOL_FACE => symbol.shape = utils::SYMBOL_FACTORY.into(),
               utils::SYMBOL_FACTORY => symbol.shape = utils::SYMBOL_BUBBLE.into(),
               utils::SYMBOL_BUBBLE => symbol.shape = utils::SYMBOL_HOUSE.into(),
               _ => continue,
            }
        }
    }

}
