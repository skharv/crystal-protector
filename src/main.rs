use bevy::{prelude::*, window::WindowResolution, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};
use bevy_pixels::{PixelsPlugin, PixelsOptions};

mod bundle;
mod component;
mod system;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 800;
pub const SCALE: i32 = 2;
pub const CHUNK_SIZE: i32 = 16;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "spreading".into(),
                resolution: WindowResolution::new(
                    WIDTH as f32,
                    HEIGHT as f32
                    ),
                    resize_constraints: WindowResizeConstraints {
                        min_width: WIDTH as f32,
                        min_height: HEIGHT as f32,
                        ..default()
                    },
                    fit_canvas_to_parent: false,
                    ..default()
            }),
            close_when_requested: true,
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
        }))
    .add_plugins(PixelsPlugin {
        primary_window: Some(PixelsOptions{
        width: (WIDTH/SCALE) as u32,
        height: (WIDTH/SCALE) as u32,
        scale_factor: SCALE as f32,
        auto_resize_buffer: true,
        auto_resize_surface: false,
        })
    })
    .add_plugins(system::GamePlugin);

    #[cfg(debug_assertions)]
    {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(LogDiagnosticsPlugin::default());
    }
        app.run();
}
