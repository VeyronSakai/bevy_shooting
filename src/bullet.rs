use crate::common::FiresBullet;
use crate::physics::*;
use crate::player::*;
use bevy::prelude::*;

pub struct Bullet;

pub fn spawn_bullet(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &FiresBullet), With<Player>>,
) {
    if let Ok((player_transform, fires_bullet)) = player_query.single_mut() {
        if fires_bullet.value {
            let sprite_size = Vec2::new(10.0, 10.0);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        size: sprite_size,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: player_transform.translation,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Bullet)
                .insert(Velocity {
                    speed: 30.0,
                    dir: Vec2::new(0.0, 1.0),
                });
        }
    }
}

pub fn update_bullet_pos(mut bullet_query: Query<(&mut Transform, &Velocity), With<Bullet>>) {
    for (mut transform, velocity) in bullet_query.iter_mut() {
        transform.translation.y += velocity.speed;
    }
}
