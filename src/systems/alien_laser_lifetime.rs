use bevy::{
    prelude::{Commands, Entity, Query, Res},
    time::Time,
};

use crate::components::alien_laser::AlienLaser;

pub fn alien_laser_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut lasers: Query<(Entity, &mut AlienLaser)>,
) {
    for (laser_entity, mut laser) in &mut lasers {
        laser.lifetime.tick(time.delta());

        if laser.lifetime.finished() {
            commands.entity(laser_entity).despawn();
        }
    }
}
