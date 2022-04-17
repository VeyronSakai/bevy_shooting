use crate::{animate_explosion_sprite, collide, explosion::ExplosionTimer, App, Bullet, BulletOwner, BulletType, Commands, Enemy, Explosion, Materials, Player, Query, Score, play_se};
use bevy::prelude::*;

// const EXPLOSION_SOUND: &str = "sounds/n148.mp3";

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_explosion_sprite)
            .add_system(player_bullet_collide_enemy)
            .add_system(enemy_bullet_collide_player);
    }
}

fn player_bullet_collide_enemy(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Sprite, &BulletOwner), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
    materials: Res<Materials>,
    mut score_query: Query<&mut Score>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (bullet_entity, bullet_transform, bullet_sprite, bullet_owner) in bullet_query.iter() {
        for (enemy_entity, enemy_transform, enemy_sprite) in enemy_query.iter() {
            if bullet_owner.bullet_type != BulletType::Player {
                continue;
            }

            let collision = collide(
                bullet_transform.translation,
                bullet_sprite.custom_size.unwrap()
                    * Vec2::new(bullet_transform.scale.x, bullet_transform.scale.y),
                enemy_transform.translation,
                Vec2::new(enemy_transform.scale.x * 5.0, enemy_transform.scale.y * 5.0),
            );

            match collision {
                Some(collision) => collision,
                None => continue,
            };

            // mp3 doesn't play
            // crate::play_se(&asset_server, &audio, EXPLOSION_SOUND);

            // spawn explosion
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: materials.explosion.clone(),
                    transform: Transform {
                        translation: enemy_transform.translation,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ExplosionTimer {
                    value: Timer::from_seconds(0.05, true),
                })
                .insert(Explosion);

            commands.entity(bullet_entity).despawn();
            commands.entity(enemy_entity).despawn();

            let mut score = score_query.single_mut();
            score.increment();
        }
    }
}

fn enemy_bullet_collide_player(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Sprite, &BulletOwner), With<Bullet>>,
    player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
    materials: Res<Materials>,
) {
    for (bullet_entity, bullet_transform, bullet_sprite, bullet_owner) in bullet_query.iter() {
        let (player_entity, player_transform, player_sprite) = match player_query.get_single() {
            Ok(x) => x,
            Err(_) => return,
        };

        if bullet_owner.bullet_type != BulletType::Enemy {
            continue;
        }

        let collision = collide(
            bullet_transform.translation,
            Vec2::new(bullet_transform.scale.x, bullet_transform.scale.y),
            player_transform.translation,
            Vec2::new(player_transform.scale.x, player_transform.scale.y),
        );

        match collision {
            Some(collision) => collision,
            None => continue,
        };

        // spawn explosion
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: materials.explosion.clone(),
                transform: Transform {
                    translation: player_transform.translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ExplosionTimer {
                value: Timer::from_seconds(0.05, true),
            })
            .insert(Explosion);

        commands.entity(bullet_entity).despawn();
        commands.entity(player_entity).despawn();
    }
}
