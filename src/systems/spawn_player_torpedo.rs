use bevy::{
    prelude::{
        info, AssetServer, Commands, Input, KeyCode, Query, Res, ResMut, Transform, Vec2, Vec3,
        With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
};

use crate::{
    components::{player::Player, player_torpedo::PlayerTorpedo},
    resources::{selected_weapon::SelectedWeapon, torpedo_ammunition::TorpedoAmmunition},
};

pub fn spawn_player_torpedo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut ammunition: ResMut<TorpedoAmmunition>,
    selected_weapon: ResMut<SelectedWeapon>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if selected_weapon.0 == 2 {
        let player_transform = player.get_single().unwrap();

        if ammunition.0 < 1 {
            info!("Out of torpedos");
            return;
        }

        let texture = asset_server.load("player_torpedo.png");

        let torpedo = PlayerTorpedo {
            speed: 300.0,
            size: Vec2::new(30.0, 30.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            lifetime: Timer::from_seconds(15.0, TimerMode::Once),
        };

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(torpedo.size),
                    ..Default::default()
                },
                transform: *player_transform,
                texture,
                ..Default::default()
            })
            .insert(torpedo);

        ammunition.0 -= 1;

        info!("Fired 1 torpedo. {:?} torpedos remaining", ammunition.0);
    }
}
