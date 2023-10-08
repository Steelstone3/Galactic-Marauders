use crate::{
    components::alien::{Alien, Direction},
    resources::{enemy_count::EnemyCount, enemy_limit::EnemyLimit},
};
use bevy::{
    prelude::{AssetServer, Commands, Res, ResMut, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

pub fn spawn_alien(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_count: ResMut<EnemyCount>,
    enemy_limit: ResMut<EnemyLimit>,
) {
    if enemy_count.0 < enemy_limit.0 {
        enemy_count.0 += 1;

        let texture = asset_server.load("enemy.png");
        let alien = Alien {
            speed: 300.0,
            size: Vec2::new(80.0, 80.0),
            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            direction: Direction::Right,
        };

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(alien.size),
                    ..Default::default()
                },
                transform: Transform {
                    scale: alien.scale,
                    ..Default::default()
                },
                texture,
                ..Default::default()
            })
            .insert(alien);
    }
}
