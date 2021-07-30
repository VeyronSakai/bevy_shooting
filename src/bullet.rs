use bevy::prelude::*;
use crate::physics::*;

pub struct Bullet;

pub fn spawn_bullet(commands: &mut Commands) {
    let sprite_size = Vec2::new(10.0, 10.0);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            size: sprite_size,
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Bullet)
        .insert(Velocity { speed: 1.0, dir: Vec2::new(0.0, 1.0) });
}

pub fn update_bullet_pos(mut bullet_query: Query<(&mut Transform, &Velocity), With<Bullet>>) {
    if let Ok((mut transform, velocity)) = bullet_query.single_mut() {
        transform.translation.y += 1.0;
    }
}