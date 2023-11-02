use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_player, spawn_enemy))
            .add_systems(
                Update,
                (
                    player_movement,
                    player_movement_box,
                    enemy_movement,
                    update_enemy_direction,
                    enemy_movement_box,
                    enemy_collide_player,
                    enemy_collide_enemy,
                ),
            );
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
        ..default()
    });
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ball_blue_large.png"),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Player,
    ));
}

pub const PLAYER_SIZE: f32 = 128.0;
pub const PLAYER_SPEED: f32 = 500.0;

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

fn player_movement_box(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

const ENEMY_SIZE: f32 = 128.0;

const ENEMY_SPEED: f32 = 200.0;

const ENEMY_AMOUNT: usize = 10;

#[derive(Component)]
struct Enemy {
    direction: Vec2,
}

fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMY_AMOUNT {
        let x =
            (random::<f32>() * window.width()).clamp(0.0 + ENEMY_SIZE, window.width() - ENEMY_SIZE);
        let y = (random::<f32>() * window.height())
            .clamp(0.0 + ENEMY_SIZE, window.height() - ENEMY_SIZE);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/ball_red_large.png"),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

fn update_enemy_direction(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        // change direction
        if translation.x < x_min || translation.x > x_max {
            println!("Change direction x!");
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            println!("Change direction y!");
            enemy.direction.y *= -1.0;
        }
    }
}

fn enemy_movement_box(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

fn enemy_collide_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let dist = player_transform
                .translation
                .distance(enemy_transform.translation);

            if dist < PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0 {
                println!("You died!");
                commands.entity(player_entity).despawn();
            }
        }
    }
}

fn enemy_collide_enemy(
    enemy_transform_query: Query<(Entity, &Transform), With<Enemy>>,
    mut enemy_query: Query<&mut Enemy>,
) {
    for (enemy_entity, enemy_transform) in enemy_transform_query.iter() {
        for (other_enemy_entity, other_enemy_transform) in enemy_transform_query.iter() {
            if enemy_entity != other_enemy_entity {
                let dist = enemy_transform
                    .translation
                    .distance(other_enemy_transform.translation);

                if dist < ENEMY_SIZE / 2.0 + ENEMY_SIZE / 2.0 {
                    println!("Collide!");
                    let collision_normal =
                        enemy_transform.translation - other_enemy_transform.translation;
                    let collision_normal =
                        Vec2::new(collision_normal.x, collision_normal.y).normalize();
                    // reflection angle
                    if let Ok(enemy) = enemy_query.get(enemy_entity) {
                        let enemy_1_reflection = (enemy.direction
                            - 2.0 * enemy.direction.dot(collision_normal) * collision_normal)
                            .normalize();

                        if let Ok(mut enemy_1_mut) = enemy_query.get_mut(enemy_entity) {
                            enemy_1_mut.direction = enemy_1_reflection;
                        }
                    }
                }
            }
        }
    }
}
