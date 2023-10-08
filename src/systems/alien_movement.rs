use crate::{
    components::alien::{Alien, Direction},
    resources::window_size::WindowSize,
};
use bevy::{
    prelude::{Mut, Query, Res, ResMut, Transform},
    time::Time,
};

pub fn alien_movement(
    mut characters: Query<(&mut Transform, &mut Alien)>,
    time: Res<Time>,
    window_size: ResMut<WindowSize>,
) {
    for (mut transform, mut alien) in &mut characters {
        let alien_speed = alien.speed * time.delta_seconds();

        alien_y_location(&window_size, &alien, &mut transform);

        if is_right_bound(window_size.width, transform.translation.x) {
            alien.direction = Direction::Left;
        }
        if is_left_bound(window_size.width, transform.translation.x) {
            alien.direction = Direction::Right;
        }

        match alien.direction {
            Direction::Left => transform.translation.x -= alien_speed,
            Direction::Right => transform.translation.x += alien_speed,
        };
    }
}

fn alien_y_location(
    window_size: &ResMut<'_, WindowSize>,
    alien: &Alien,
    transform: &mut Mut<'_, Transform>,
) {
    let spawn_area = window_size.height / 2.0;
    let spawn_location = spawn_area - alien.size.y / 2.0 * alien.scale.y - 20.0;

    transform.translation.y = spawn_location;
}

fn is_left_bound(window_width: f32, horizontal_position: f32) -> bool {
    -horizontal_position > window_width / 2.0
}

fn is_right_bound(window_width: f32, horizontal_position: f32) -> bool {
    horizontal_position > window_width / 2.0
}
