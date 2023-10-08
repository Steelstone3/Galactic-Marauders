use bevy::{
    prelude::{
        info, AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
};

use crate::{
    components::{laser::Laser, player::Player},
    resources::{laser_ammunition::LaserAmmunition, selected_weapon::SelectedWeapon},
};

pub fn spawn_laser(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<LaserAmmunition>,
    selected_weapon: ResMut<SelectedWeapon>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if selected_weapon.0 == 1 {
        let player_transform = player.get_single().unwrap();

        if ammunition.0 < 1.0 {
            info!("Out of laser charge");
            return;
        }

        let texture = asset_server.load("player_laser.png");

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                transform: *player_transform,
                texture,
                ..Default::default()
            })
            .insert(Laser {
                speed: 0.0,
                lifetime: Timer::from_seconds(10.0, TimerMode::Once),
            });

        ammunition.0 -= 1.0;

        info!(
            "Used 1 laser charge. {:?} laser charge remaining",
            ammunition.0
        );
    }
}
