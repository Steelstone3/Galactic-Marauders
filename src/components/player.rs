use bevy::{
    ecs::component::Component,
    prelude::{Vec2, Vec3},
};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub size: Vec2,
    pub scale: Vec3,
}
