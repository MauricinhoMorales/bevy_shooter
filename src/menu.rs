use bevy::prelude::*;

pub struct MenuPlugin;

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy, States, Hash)]
pub enum GameState {
    Menu,
    InGame,
    GameOver,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Menu)
            .add_systems(Startup, setup_scene)
            .add_systems(Update, start_game);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

fn start_game(
    keys: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *game_state == (GameState::GameOver) {
        if keys.pressed(KeyCode::KeyP) {
            next_state.set(GameState::InGame);
        }
    }
    if *game_state == GameState::Menu {
        if keys.pressed(KeyCode::KeyP) {
            next_state.set(GameState::InGame);
        }
    }
}
