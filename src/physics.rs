use bevy::math::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub speed: f32,
    pub dir: Vec2,
}
