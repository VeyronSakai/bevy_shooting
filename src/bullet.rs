use crate::common::*;
use crate::enemy::*;
use crate::physics::*;
use crate::player::*;
use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_bullet_pos)
            .add_system(spawn_player_bullet)
            .add_system(spawn_enemy_bullet)
            .add_system(despawn_bullet);
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletOwner {
    pub bullet_type: BulletType,
}

#[derive(PartialEq, Copy, Clone)]
pub enum BulletType {
    Player,
    Enemy,
}

pub fn spawn_player_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut FireBulletInfo, &Sprite), With<Player>>,
) {
    let (player_transform, mut fires_bullet, player_sprite) = match player_query.get_single_mut() {
        Ok(x) => x,
        Err(_) => return,
    };

    if fires_bullet.can_fire {
        fires_bullet.time += time.delta_seconds();

        if fires_bullet.is_under_suspension() {
            return;
        }

        fires_bullet.time = 0.0;

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        player_transform.translation.x,
                        player_transform.translation.y + player_transform.scale.y / 2.0,
                        0.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Bullet)
            .insert(Velocity {
                speed: 10.0,
                dir: Vec2::new(0.0, 1.0),
            })
            .insert(BulletOwner {
                bullet_type: BulletType::Player,
            });
    }
}

pub fn spawn_enemy_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(&Transform, &mut FireBulletInfo, &Sprite), With<Enemy>>,
) {
    for (enemy_transform, mut fires_bullet, enemy_sprite) in enemy_query.iter_mut() {
        if fires_bullet.can_fire {
            fires_bullet.time += time.delta_seconds();

            if fires_bullet.is_under_suspension() {
                continue;
            }

            fires_bullet.time = 0.0;

            let sprite_size = Vec2::new(10.0, 10.0);

            spawn_enemy_bullet_internal(
                &mut commands,
                sprite_size,
                enemy_transform,
                enemy_sprite,
                Vec2::new(0., -1.),
            );

            spawn_enemy_bullet_internal(
                &mut commands,
                sprite_size,
                enemy_transform,
                enemy_sprite,
                Vec2::new(-1., -1.),
            );

            spawn_enemy_bullet_internal(
                &mut commands,
                sprite_size,
                enemy_transform,
                enemy_sprite,
                Vec2::new(1.0, -1.0),
            );
        }
    }
}

fn spawn_enemy_bullet_internal(
    commands: &mut Commands,
    sprite_size: Vec2,
    enemy_transform: &Transform,
    enemy_sprite: &Sprite,
    dir: Vec2,
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(sprite_size),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    enemy_transform.translation.x,
                    enemy_transform.translation.y,
                    0.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bullet)
        .insert(Velocity { speed: 1.5, dir })
        .insert(BulletOwner {
            bullet_type: BulletType::Enemy,
        });
}

pub fn update_bullet_pos(mut bullet_query: Query<(&mut Transform, &Velocity), With<Bullet>>) {
    for (mut transform, velocity) in bullet_query.iter_mut() {
        transform.translation.x += velocity.dir.x * velocity.speed;
        transform.translation.y += velocity.dir.y * velocity.speed;
    }
}

pub fn despawn_bullet(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    mut bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in bullet_query.iter_mut() {
        if transform.translation.y < -window_size.h || window_size.h < transform.translation.y {
            commands.entity(entity).despawn();
        }
    }
}
