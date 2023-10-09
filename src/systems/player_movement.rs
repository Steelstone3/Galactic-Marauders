use bevy::{
    prelude::{Input, KeyCode, Mut, Query, Res, ResMut, Transform},
    time::Time,
};

use crate::{components::player::Player, resources::window_size::WindowSize};

pub fn player_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    window_size: ResMut<WindowSize>,
) {
    for (mut transform, player) in &mut characters {
        let player_speed = player.speed * time.delta_seconds();

        player_y_location(&window_size, player, &mut transform);

        // Right
        if input.pressed(KeyCode::D) && !is_right_bound(window_size.width, transform.translation.x)
        {
            transform.translation.x += player_speed;
        }
        // Left
        else if input.pressed(KeyCode::A)
            && !is_left_bound(window_size.width, transform.translation.x)
        {
            transform.translation.x -= player_speed;
        }
    }
}

fn player_y_location(
    window_size: &ResMut<'_, WindowSize>,
    player: &Player,
    transform: &mut Mut<'_, Transform>,
) {
    let spawn_area = -window_size.height / 2.0;
    let spawn_location = spawn_area + player.size.y / 2.0 * player.scale.y + 20.0;

    transform.translation.y = spawn_location;
}

fn is_left_bound(window_width: f32, horizontal_position: f32) -> bool {
    -horizontal_position > window_width / 2.0
}

fn is_right_bound(window_width: f32, horizontal_position: f32) -> bool {
    horizontal_position > window_width / 2.0
}
