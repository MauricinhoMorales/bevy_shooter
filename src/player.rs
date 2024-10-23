use bevy::prelude::*;

use crate::alien;
use crate::menu::GameState;
use crate::projectile;
use crate::resolution;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_player)
            .add_systems(OnExit(GameState::InGame), despawn_player)
            .add_systems(
                Update,
                (update_player, update_player_collision).run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
struct Player {
    pub shoot_timer: f32,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    let player_image = asset_server.load("player.png");
    commands.spawn((
        SpriteBundle {
            texture: player_image,
            transform: Transform::from_xyz(
                0.,
                -(resolution.screen_dimensions.y * 0.5) + (resolution.pixel_ratio * 5.0),
                0.,
            )
            .with_scale(Vec3::splat(resolution.pixel_ratio)),
            ..Default::default()
        },
        Player { shoot_timer: 0. },
    ));
}

fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

const SPEED: f32 = 200.;
const BULLET_SPEED: f32 = 400.;
const SHOOT_COOLDOWN: f32 = 0.5;
fn update_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<resolution::Resolution>,
    game_state: Res<State<GameState>>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    if *game_state == GameState::InGame {
        let mut horizontal = 0.;

        let mut vertical = 0.;

        if keys.pressed(KeyCode::KeyA) {
            horizontal += -1.;
        }
        if keys.pressed(KeyCode::KeyD) {
            horizontal += 1.;
        }
        if keys.pressed(KeyCode::KeyS) {
            vertical += -1.;
        }
        if keys.pressed(KeyCode::KeyW) {
            vertical += 1.;
        }

        transform.translation.x += horizontal * time.delta_seconds() * SPEED;
        transform.translation.y += vertical * time.delta_seconds() * SPEED;

        //confine player
        let left_bound = -resolution.screen_dimensions.x * 0.5;
        let right_bound = resolution.screen_dimensions.x * 0.5;

        if transform.translation.x > right_bound {
            transform.translation.x = right_bound;
        }
        if transform.translation.x < left_bound {
            transform.translation.x = left_bound;
        }
        let down_bound = -resolution.screen_dimensions.y * 0.5;
        let up_bound = resolution.screen_dimensions.y * 0.5;

        if transform.translation.y > up_bound {
            transform.translation.y = up_bound;
        }
        if transform.translation.y < down_bound {
            transform.translation.y = down_bound;
        }

        player.shoot_timer -= time.delta_seconds();

        if keys.pressed(KeyCode::Space) && player.shoot_timer <= 0. {
            player.shoot_timer = SHOOT_COOLDOWN;
            let bullet_texture = asset_server.load("bullet.png");
            commands.spawn((
                SpriteBundle {
                    texture: bullet_texture,
                    transform: Transform::from_translation(transform.translation)
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    ..Default::default()
                },
                projectile::Projectile {
                    speed: BULLET_SPEED,
                },
            ));
        }
    }
}

const PLAYER_RADIUS: f32 = 24.;

fn update_player_collision(
    mut alien_query: Query<(&mut alien::Alien, &Transform), Without<alien::Dead>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *game_state == GameState::InGame {
        for (mut _alien, alien_transform) in alien_query.iter_mut() {
            for (_player_entity, player_transform) in player_query.iter_mut() {
                let player_pos = Vec2::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                );
                let alien_pos =
                    Vec2::new(alien_transform.translation.x, alien_transform.translation.y);
                if Vec2::distance(alien_pos, player_pos) < PLAYER_RADIUS {
                    next_state.set(GameState::GameOver);
                }
            }
        }
    }
}
