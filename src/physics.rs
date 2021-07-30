use bevy::math::*;
use bevy::ecs::prelude::*;
use bevy::sprite::prelude::*;
use bevy::prelude::*;

pub struct Velocity {
    pub speed: f32,
    pub dir: Vec2,
}
