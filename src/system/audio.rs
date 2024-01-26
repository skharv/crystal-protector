use bevy::{prelude::*, audio::{PlaybackMode, Volume}};

use crate::component;

pub fn play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>
    ) {
    commands.spawn((AudioBundle{
        source: asset_server.load("bgm.ogg"),
        settings: PlaybackSettings{
            mode: PlaybackMode::Loop,
            volume: Volume::new_relative(0.5),
            ..default()
        },
        ..default()
    },
    component::Music));
}

