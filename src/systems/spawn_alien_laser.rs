use bevy::{
    prelude::{AssetServer, Commands, Input, KeyCode, Query, Res, Transform, Vec2, Vec3, With},
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
};

use crate::components::{alien::Alien, alien_laser::AlienLaser};

pub fn spawn_alien_laser(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    alien: Query<&Transform, With<Alien>>,
) {
    if !input.just_pressed(KeyCode::T) {
        return;
    }

    let alien_transform = alien.get_single().unwrap();

    let texture = asset_server.load("enemy_laser.png");

    let laser = AlienLaser {
        speed: -300.0,
        lifetime: Timer::from_seconds(5.0, TimerMode::Once),
        size: Vec2::new(20.0, 20.0),
        scale: Vec3::new(1.0, 1.0, 1.0),
    };

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(laser.size),
                ..Default::default()
            },
            transform: *alien_transform,
            texture,
            ..Default::default()
        })
        .insert(laser);
}
