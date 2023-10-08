use bevy::{ecs::component::Component, time::Timer, prelude::{Vec3, Vec2}};

#[derive(Component)]
pub struct Torpedo {
    pub speed: f32,
    pub size: Vec2,
    pub scale: Vec3,
    pub lifetime: Timer,
}
