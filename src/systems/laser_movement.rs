use bevy::{
    prelude::{Query, Res, Transform},
    time::Time,
};
use crate::components::laser::Laser;

pub fn laser_movement(
    mut characters: Query<(&mut Transform, &Laser)>,
    time: Res<Time>,
) {
    for (mut transform, laser) in &mut characters {
        let laser_speed = laser.speed * time.delta_seconds();

        transform.translation.y += laser_speed;
    }
}

