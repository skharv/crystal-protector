use bevy::prelude::*;

use crate::bundle;
use crate::component;
use crate::utils;

pub fn generate_start_symbols(
    mut commands: Commands
    ) {
    let you_position = Vec2::new(((crate::WIDTH/crate::SCALE) - crate::SYMBOL_SIZE as i32) as f32 / 2.0, (((crate::HEIGHT/crate::SCALE) - crate::MENU_SIZE) - crate::SYMBOL_SIZE as i32) as f32 / 2.0);
    let wasd_position = Vec2::new((((crate::WIDTH/crate::SCALE) / 2) - crate::SYMBOL_SIZE as i32) as f32 / 2.0, (((crate::HEIGHT/crate::SCALE) / 2) - crate::SYMBOL_SIZE as i32) as f32);
    let click_position = Vec2::new((((crate::WIDTH/crate::SCALE) / 2) - crate::SYMBOL_SIZE as i32) as f32 / 2.0 + (((crate::WIDTH/crate::SCALE) / 2) - crate::SYMBOL_SIZE as i32) as f32, (((crate::HEIGHT/crate::SCALE) / 2) - crate::SYMBOL_SIZE as i32) as f32);
    let offset = (crate::SYMBOL_SIZE + 2) as f32;

    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: wasd_position.x, y: wasd_position.y - offset },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_W.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: wasd_position.x - offset, y: wasd_position.y },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_A.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: wasd_position.x, y: wasd_position.y },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_S.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: wasd_position.x + offset, y: wasd_position.y },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_D.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x, y: click_position.y },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_L_BUTTON.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x + offset, y: click_position.y },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_R_BUTTON.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x - offset, y: click_position.y },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_LASER.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x + offset * 2.0, y: click_position.y }, 
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_BOMB.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x + offset * 3.0, y: click_position.y}, 
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_SQUIGGLE.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x + offset * 3.0, y: click_position.y - offset}, 
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_1.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x + offset * 2.0, y: click_position.y - offset }, 
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_HOUSE.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x + offset * 2.0, y: click_position.y + offset }, 
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_FACE.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x + offset * 3.0, y: click_position.y + offset }, 
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_SQUIGGLE.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x + offset * 2.0, y: click_position.y + offset * 2.0}, 
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_BUBBLE.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: click_position.x + offset * 3.0, y: click_position.y + offset * 2.0}, 
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_4.into() },
        ui: component::Ui
    }).insert(component::Start);

    for i in 0..utils::CRYSTAL_COUNT {
        commands.spawn(bundle::SymbolBundle {
            position: component::Position { x: ((crate::WIDTH/crate::SCALE) - (12 + (8 * i))) as f32, y: ((crate::HEIGHT/crate::SCALE) - 12) as f32 },
            colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
            symbol: component::Symbol { shape: utils::SYMBOL_OUTER_CRYSTAL.into() },
            ui: component::Ui
        }).insert(component::Start);
    }
    commands.spawn(bundle::SymbolBundle {
            position: component::Position { x: ((crate::WIDTH/crate::SCALE) - 36) as f32, y: ((crate::HEIGHT/crate::SCALE) - 12) as f32 - offset },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_ARROW.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: ((crate::WIDTH/crate::SCALE) - 36) as f32, y: ((crate::HEIGHT/crate::SCALE) - 12) as f32 - (offset * 2.0) },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_HEART.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: you_position.x, y: you_position.y - offset}, 
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_ARROW.into() },
        ui: component::Ui
    }).insert(component::Start);
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: you_position.x, y: you_position.y}, 
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_YOU.into() },
        ui: component::Ui
    }).insert(component::Start);
}

pub fn generate_symbols(
    mut commands: Commands
    ) {
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: (((crate::WIDTH/crate::SCALE) / 2) - crate::SYMBOL_SIZE as i32) as f32, y: (((crate::HEIGHT/crate::SCALE) / 2) - crate::SYMBOL_SIZE as i32) as f32 },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_SAD.into() },
        ui: component::Ui
    }).insert(component::Defeat);

    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: (((crate::WIDTH/crate::SCALE) / 2) - crate::SYMBOL_SIZE as i32) as f32, y: (((crate::HEIGHT/crate::SCALE) / 2) - crate::SYMBOL_SIZE as i32) as f32 },
        colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
        symbol: component::Symbol { shape: utils::SYMBOL_HAPPY.into() },
        ui: component::Ui
    }).insert(component::Victory);

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

    for i in 0..utils::CRYSTAL_COUNT {
        commands.spawn(bundle::SymbolBundle {
            position: component::Position { x: ((crate::WIDTH/crate::SCALE) - (12 + (8 * i))) as f32, y: ((crate::HEIGHT/crate::SCALE) - 12) as f32 },
            colour: component::Colour { r: utils::COLOUR_UI[0], g: utils::COLOUR_UI[1], b: utils::COLOUR_UI[2], a: utils::COLOUR_UI[3] },
            symbol: component::Symbol { shape: utils::SYMBOL_OUTER_CRYSTAL.into() },
            ui: component::Ui
        }).insert(component::Crystal{ id: i });
    }
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
                utils::Action::Bubble => symbol.shape = utils::SYMBOL_BUBBLE.into(),
                utils::Action::House => symbol.shape = utils::SYMBOL_HOUSE.into(),
            }
        }
    }
}

