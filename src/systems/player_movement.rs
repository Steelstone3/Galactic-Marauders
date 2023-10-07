use bevy::{
    prelude::{Input, KeyCode, Query, Res, Transform},
    time::Time,
};

use crate::components::player::Player;

pub fn player_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let player_speed = player.speed * time.delta_seconds();
        let player_diagonal_speed = calculate_diagonal_speed(player_speed);

        // Forward right diagonal
        if input.pressed(KeyCode::W) && input.pressed(KeyCode::D) {
            transform.translation.y += player_diagonal_speed;
            transform.translation.x += player_diagonal_speed;
        }
        // Forward left diagonal
        else if input.pressed(KeyCode::W) && input.pressed(KeyCode::A) {
            transform.translation.y += player_diagonal_speed;
            transform.translation.x -= player_diagonal_speed;
        }
        // Downward right diagonal
        else if input.pressed(KeyCode::S) && input.pressed(KeyCode::D) {
            transform.translation.y -= player_diagonal_speed;
            transform.translation.x += player_diagonal_speed;
        }
        // Downward left diagonal
        else if input.pressed(KeyCode::S) && input.pressed(KeyCode::A) {
            transform.translation.y -= player_diagonal_speed;
            transform.translation.x -= player_diagonal_speed;
        }
        // Forward
        else if input.pressed(KeyCode::W) {
            transform.translation.y += player_speed;
        }
        // Downward
        else if input.pressed(KeyCode::S) {
            transform.translation.y -= player_speed;
        }
        // Right
        else if input.pressed(KeyCode::D) {
            transform.translation.x += player_speed;
        }
        // Left
        else if input.pressed(KeyCode::A) {
            transform.translation.x -= player_speed;
        }
    }
}

fn calculate_diagonal_speed(speed: f32) -> f32 {
    speed / 4.0 * 3.0
}
