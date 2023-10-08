use crate::components::torpedo::Torpedo;
use bevy::{
    prelude::{Query, Res, Transform},
    time::Time,
};

pub fn torpedo_movement(mut characters: Query<(&mut Transform, &Torpedo)>, time: Res<Time>) {
    for (mut transform, torpedo) in &mut characters {
        let torpedo_speed = torpedo.speed * time.delta_seconds();

        transform.translation.y += torpedo_speed;
    }
}
