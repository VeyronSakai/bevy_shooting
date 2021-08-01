mod bullet;
mod common;
mod enemy;
mod physics;
mod player;

use crate::bullet::*;
use crate::common::*;
use crate::enemy::*;
use crate::physics::*;
use crate::player::*;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

const PLAYER_SPRITE: &str = "player.png";
const ENEMY_SPRITE: &str = "enemy.png";

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "shooting".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(PlayerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(EnemyPlugin)
        .add_system_to_stage(CoreStage::PreUpdate, handle_input.system())
        .add_system(bullet_collide.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.get_primary_mut().unwrap();

    // カメラを生成する
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // 背景の色を黒くする
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

    commands.insert_resource(WindowSize {
        w: window.width(),
        h: window.height(),
    });

    commands.insert_resource(Materials {
        player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        enemy: materials.add(asset_server.load(ENEMY_SPRITE).into()),
    });
}

fn handle_input(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut FireBulletInfo), With<Player>>,
) {
    if let Ok((mut velocity, mut fires_bullet)) = player_query.single_mut() {
        velocity.dir = Vec2::new(0.0, 0.0);

        if input.pressed(KeyCode::W) {
            velocity.dir.y += 1.0;
        }

        if input.pressed(KeyCode::S) {
            velocity.dir.y += -1.0;
        }

        if input.pressed(KeyCode::D) {
            velocity.dir.x += 1.0;
        }

        if input.pressed(KeyCode::A) {
            velocity.dir.x += -1.0;
        }

        fires_bullet.can_fire = input.pressed(KeyCode::Space);
    }
}

fn bullet_collide(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Sprite), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform, bullet_sprite) in bullet_query.iter() {
        if let Ok((enemy_entity, enemy_transform, enemy_sprite)) = enemy_query.single() {
            let collision = collide(
                bullet_transform.translation,
                bullet_sprite.size * Vec2::from(bullet_transform.scale),
                enemy_transform.translation,
                enemy_sprite.size * Vec2::from(enemy_transform.scale),
            );

            match collision {
                Some(collision) => collision,
                None => continue,
            };

            commands.entity(bullet_entity).despawn();
            commands.entity(enemy_entity).despawn();
        }
    }
}
