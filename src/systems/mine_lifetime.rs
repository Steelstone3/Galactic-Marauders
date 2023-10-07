use bevy::{
    prelude::{info, Commands, Entity, Query, Res, ResMut},
    time::Time,
};

use crate::{components::mine::Mine, resources::mine_ammunition::MineAmmunition};

pub fn mine_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut mines: Query<(Entity, &mut Mine)>,
    mut mine_ammunition: ResMut<MineAmmunition>,
) {
    for (mine_entity, mut mine) in &mut mines {
        mine.lifetime.tick(time.delta());

        if mine.lifetime.finished() {
            commands.entity(mine_entity).despawn();

            mine_ammunition.0 += 1.0;

            info!("Mine recovered Current mines: {:?}", mine_ammunition.0);
        }
    }
}
