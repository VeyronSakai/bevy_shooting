use bevy::ecs::prelude::*;
use bevy::prelude::*;

pub struct Player;

pub struct Velocity {
    pub val: Vec2,
}

pub fn spawn_player(commands: &mut Commands) {
    let sprite_size = Vec2::new(100.0, 100.0);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            size: sprite_size,
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Player)
        .insert(Velocity { val: Vec2::new(0.0, 0.0) });
}