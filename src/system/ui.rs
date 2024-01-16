use bevy::prelude::*;

use crate::bundle;
use crate::component;

pub fn generate_symbol(
    mut commands: Commands
    ) {

    let array = [[false, false, true, true, true, true, false, false],
                 [false, true, false, false, false, false, true, false],
                 [false, true, false, false, false, false, true, false],
                 [false, true, false, false, false, false, true, false],
                 [false, true, false, false, false, false, true, false],
                 [false, true, false, false, false, false, true, false],
                 [false, true, false, false, false, false, true, false],
                 [false, false, true, true, true, true, false, false]];

    commands.spawn(bundle::SymbolBundle {
        position: component::Position { x: 10.0, y: 10.0 },
        colour: component::Colour { r: 255, g: 255, b: 255, a: 255 },
        symbol: component::Symbol { shape: array },
        ui: component::Ui
    });
}
