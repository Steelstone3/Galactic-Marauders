use bevy::{
    prelude::{info, Commands, Entity, Query, Res, ResMut},
    time::Time,
};

use crate::{components::torpedo::Torpedo, resources::torpedo_ammunition::TorpedoAmmunition};

pub fn torpedo_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut torpedos: Query<(Entity, &mut Torpedo)>,
    mut ammunition: ResMut<TorpedoAmmunition>,
) {
    for (torpedo_entity, mut torpedo) in &mut torpedos {
        torpedo.lifetime.tick(time.delta());

        if torpedo.lifetime.finished() {
            commands.entity(torpedo_entity).despawn();

            ammunition.0 += 1.0;

            info!("Torpedos recovered Current torpedos: {:?}", ammunition.0);
        }
    }
}
