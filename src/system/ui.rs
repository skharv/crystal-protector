use bevy::prelude::*;

use crate::bundle;
use crate::component;

const UI_COLOR: [u8; 4] = [255, 235, 153, 255];

pub fn generate_symbols(
    mut commands: Commands
    ) {
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: 10.0, y: 10.0 },
        colour: component::Colour { r: UI_COLOR[0], g: UI_COLOR[1], b: UI_COLOR[2], a: UI_COLOR[3] },
        symbol: component::Symbol { shape: "08181C3C3466663C".into() },
        ui: component::Ui
    });
    
    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: 50.0, y: 50.0 },
        colour: component::Colour { r: UI_COLOR[0], g: UI_COLOR[1], b: UI_COLOR[2], a: UI_COLOR[3] },
        symbol: component::Symbol { shape: "3C4242424242423C".into() },
        ui: component::Ui
    });
}
