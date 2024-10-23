use bevy::prelude::*;

pub mod alien;
pub mod game;
pub mod menu;
pub mod player;
pub mod projectile;
pub mod resolution;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("VXR Shooter"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(1080., 720.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            game::GamePlugin,
            menu::MenuPlugin,
        ))
        .run();
}
