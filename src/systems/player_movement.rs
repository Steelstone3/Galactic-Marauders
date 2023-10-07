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

        // Right
        if input.pressed(KeyCode::D) {
            transform.translation.x += player_speed;
        }
        // Left
        else if input.pressed(KeyCode::A) {
            transform.translation.x -= player_speed;
        }
    }
}
