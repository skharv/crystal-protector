use bevy::prelude::*;

use crate::bundle;
use crate::component;
use crate::utils;

pub fn generate_symbols(
    mut commands: Commands
    ) {
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: 4.0, y: ((crate::HEIGHT/crate::SCALE) - 12) as f32 },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_RESOURCE.into() },
        ui: component::Ui
    });

    commands.spawn(bundle::BarBundle {
        position: component::Position { x: 16.0, y: ((crate::HEIGHT/crate::SCALE) - 12) as f32 },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        bar: component::Bar { percent: 25.0, cost: 0.0 },
        size: component::Size { width: 64, height: 8 },
        ui: component::Ui
    });
    
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: 84.0, y: ((crate::HEIGHT/crate::SCALE) - 12) as f32 },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_HOUSE.into() },
        ui: component::Ui
    }).insert(component::ActionUi);
    
    commands.spawn(bundle::RectBundle {
        position: component::Position { x: 0.0, y: ((crate::HEIGHT/crate::SCALE) - crate::MENU_SIZE) as f32 },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        size: component::Size { width: (crate::WIDTH/crate::SCALE), height: crate::MENU_SIZE },
        ui: component::Ui
    });
 }

pub fn update_bars(
    mut action_query: Query<&mut component::Action, With<component::Input>>,
    mut bar_query: Query<&mut component::Bar, With<component::Ui>>,
    absorb_query: Query<&component::Resources, With<component::Absorb>>
    ) {
    for resources in absorb_query.iter() {
        for mut bar in bar_query.iter_mut() {
            for action in action_query.iter_mut() {
                bar.cost = ((action.action as i32) as f32 / resources.maximum as f32) * 100.0;
            }
            bar.percent = (resources.amount as f32 / resources.maximum as f32) * 100.0;
        }
    }
}

pub fn update_action(
    mut action_query: Query<&mut component::Action, With<component::Input>>,
    mut symbol_query: Query<&mut component::Symbol, (With<component::Ui>, With<component::ActionUi>)>
) {
    for action in action_query.iter_mut() {
        for mut symbol in symbol_query.iter_mut() {
            match action.action {
                utils::Action::Bomb => symbol.shape = utils::SYMBOL_BOMB.into(),
                utils::Action::Face => symbol.shape = utils::SYMBOL_FACE.into(),
                utils::Action::Factory => symbol.shape = utils::SYMBOL_FACTORY.into(),
                utils::Action::Bubble => symbol.shape = utils::SYMBOL_BUBBLE.into(),
                utils::Action::House => symbol.shape = utils::SYMBOL_HOUSE.into(),
            }
        }
    }
}
