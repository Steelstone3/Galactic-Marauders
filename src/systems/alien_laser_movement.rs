use crate::components::alien_laser::AlienLaser;
use bevy::{
    prelude::{Query, Res, Transform},
    time::Time,
};

pub fn alien_laser_movement(mut characters: Query<(&mut Transform, &AlienLaser)>, time: Res<Time>) {
    for (mut transform, laser) in &mut characters {
        let laser_speed = laser.speed * time.delta_seconds();

        transform.translation.y += laser_speed;
    }
}
