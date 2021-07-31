use crate::common::*;
use crate::physics::*;
use crate::player::*;
use bevy::prelude::*;

pub struct Bullet;

pub fn spawn_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut FireBulletInfo), With<Player>>,
) {
    if let Ok((player_transform, mut fires_bullet)) = player_query.single_mut() {
        if fires_bullet.can_fire {

            fires_bullet.time += time.delta_seconds();

            if fires_bullet.is_in_interval() {
                return;
            }

            fires_bullet.time = 0.0;

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
                    speed: 10.0,
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

pub fn despawn_bullet(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    mut bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in bullet_query.iter_mut() {
        if transform.translation.y < -window_size.h
            || window_size.h - 100.0 < transform.translation.y
        {
            commands.entity(entity).despawn();
        }
    }
}
