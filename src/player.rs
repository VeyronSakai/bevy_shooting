use bevy::ecs::prelude::*;
use bevy::prelude::*;
use crate::physics::*;
use crate::common::*;

const SPEED: f32 = 20.0;

pub struct Player;

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
        .insert(Velocity { dir: Vec2::new(0.0, 0.0), speed: SPEED })
        .insert(FireBulletInfo{can_fire: false, time: 0.0, interval: 0.1});
}

pub fn update_player_pos(mut player_query: Query<(&mut Transform, &Velocity), With<Player>>) {
    if let Ok((mut transform, velocity)) = player_query.single_mut() {
        transform.translation.x += velocity.dir.x * velocity.speed * 0.2;
        transform.translation.y += velocity.dir.y * velocity.speed * 0.2;
    }
}