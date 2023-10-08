use bevy::{
    prelude::{AssetServer, Commands, Res, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use crate::components::player::Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("player.png");
    let player = Player {
        speed: 400.0,
        size: Vec2::new(100.0, 100.0),
        scale: Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    };

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(player.size),
                ..Default::default()
            },
            transform: Transform {
                ..Default::default()
            },
            texture,
            ..Default::default()
        })
        .insert(player);
}
