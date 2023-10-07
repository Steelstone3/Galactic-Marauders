use bevy::{
    prelude::{info, Commands, Entity, Query, Res, ResMut},
    time::Time,
};

use crate::{components::laser::Laser, resources::laser_ammunition::LaserAmmunition};

pub fn laser_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut lasers: Query<(Entity, &mut Laser)>,
    mut ammunition: ResMut<LaserAmmunition>,
) {
    for (laser_entity, mut laser) in &mut lasers {
        laser.lifetime.tick(time.delta());

        if laser.lifetime.finished() {
            commands.entity(laser_entity).despawn();

            ammunition.0 += 1.0;

            info!(
                "Laser charge recovered Current laser charge: {:?}",
                ammunition.0
            );
        }
    }
}
