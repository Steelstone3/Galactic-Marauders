use bevy::{
    ecs::component::Component,
    prelude::{Vec2, Vec3},
    time::Timer,
};

#[derive(Component)]
pub struct Torpedo {
    pub speed: f32,
    pub size: Vec2,
    pub scale: Vec3,
    pub lifetime: Timer,
}
