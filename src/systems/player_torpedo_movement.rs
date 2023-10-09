use crate::components::player_torpedo::PlayerTorpedo;
use bevy::{
    prelude::{Query, Res, Transform},
    time::Time,
};

pub fn player_torpedo_movement(
    mut characters: Query<(&mut Transform, &PlayerTorpedo)>,
    time: Res<Time>,
) {
    for (mut transform, torpedo) in &mut characters {
        let torpedo_speed = torpedo.speed * time.delta_seconds();

        transform.translation.y += torpedo_speed;
    }
}
