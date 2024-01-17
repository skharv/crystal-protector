use bevy::prelude::*;

use crate::bundle;
use crate::component;

pub fn generate_symbol(
    mut commands: Commands
    ) {
    let array = [['3', 'C'],
                ['4', '2'],
                ['4', '2'],
                ['4', '2'],
                ['4', '2'],
                ['4', '2'],
                ['4', '2'],
                ['3', 'C']];

    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: 10.0, y: 10.0 },
        colour: component::Colour { r: 255, g: 255, b: 255, a: 255 },
        symbol: component::Symbol { shape: array },
        ui: component::Ui
    });
}
