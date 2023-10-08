use bevy::{
    ecs::component::Component,
    prelude::{Vec2, Vec3},
};

pub enum Direction {
    Left,
    Right
}

#[derive(Component)]
pub struct Alien {
    pub speed: f32,
    pub size: Vec2,
    pub scale: Vec3,
    pub direction: Direction
}
