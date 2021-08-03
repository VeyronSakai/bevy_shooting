mod bullet;
mod common;
mod enemy;
mod explosion;
mod physics;
mod player;

use crate::bullet::*;
use crate::common::*;
use crate::enemy::*;
use crate::explosion::*;
use crate::physics::*;
use crate::player::*;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

const PLAYER_SPRITE: &str = "player.png";
const ENEMY_SPRITE: &str = "enemy.png";
const EXPLOSION_SHEET: &str = "explosion.png";

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "shooting".to_string(),
            width: 480.,
            height: 640.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(PlayerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(EnemyPlugin)
        .add_system_to_stage(CoreStage::PreUpdate, handle_input.system())
        .add_system(bullet_collide.system())
        .add_system(animate_explosion_sprite.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 4);

    commands.insert_resource(Materials {
        player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        enemy: materials.add(asset_server.load(ENEMY_SPRITE).into()),
        explosion: texture_atlases.add(texture_atlas),
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
    bullet_query: Query<(Entity, &Transform, &Sprite, &BulletOwner), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
    materials: Res<Materials>,
) {
    for (bullet_entity, bullet_transform, bullet_sprite, bullet_owner) in bullet_query.iter() {
        for (enemy_entity, enemy_transform, enemy_sprite) in enemy_query.iter(){

            if bullet_owner.bullet_type != BulletType::Player{
                continue;
            }

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
                .insert(Timer::from_seconds(0.05, true))
                .insert(Explosion);

            commands.entity(bullet_entity).despawn();
            commands.entity(enemy_entity).despawn();
        }
    }
}
