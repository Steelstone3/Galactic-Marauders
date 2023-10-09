use crate::components::player_laser::PlayerLaser;
use bevy::{
    prelude::{Query, Res, Transform},
    time::Time,
};

pub fn player_laser_movement(
    mut characters: Query<(&mut Transform, &PlayerLaser)>,
    time: Res<Time>,
) {
    for (mut transform, laser) in &mut characters {
        let laser_speed = laser.speed * time.delta_seconds();

        transform.translation.y += laser_speed;
    }
}
