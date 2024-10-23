use bevy::prelude::*;

use crate::alien;
use crate::menu::GameState;
use crate::player;
use crate::projectile;
use crate::resolution;
pub struct GamePlugin;

#[derive(Component)]
struct GameOverText;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            alien::AlienPlugin,
            resolution::ResolutionPlugin,
            player::PlayerPlugin,
            projectile::ProjectilePlugin,
        ))
        .add_systems(OnEnter(GameState::GameOver), display_game_over)
        .add_systems(OnExit(GameState::GameOver), despawn_game_over_text);
    }
}

fn display_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Game Over!");
    commands.spawn((
        TextBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(500.0),
                    right: Val::Px(360.0),
                    top: Val::Px(340.0),
                    bottom: Val::Px(540.0),
                },
                ..default()
            },
            text: Text::from_section(
                "Game Over!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        },
        GameOverText,
    ));
}

fn despawn_game_over_text(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
