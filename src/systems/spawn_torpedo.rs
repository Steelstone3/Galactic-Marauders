use bevy::{
    prelude::{
        info, AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
};

use crate::{
    components::{player::Player, torpedo::Torpedo},
    resources::torpedo_ammunition::TorpedoAmmunition,
};

pub fn spawn_torpedo(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<TorpedoAmmunition>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::T) {
        return;
    }

    let player_transform = player.get_single().unwrap();

    if ammunition.0 < 1.0 {
        info!("Out of torpedos");
        return;
    }

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(15.0, 15.0)),
                color: bevy::prelude::Color::hex("FF8700").unwrap(),
                ..Default::default()
            },
            transform: *player_transform,
            // texture,
            ..Default::default()
        })
        .insert(Torpedo {
            speed: 0.0,
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        });

    ammunition.0 -= 1.0;
    info!("Fired 1 torpedo. {:?} torpedos remaining", ammunition.0);
}
