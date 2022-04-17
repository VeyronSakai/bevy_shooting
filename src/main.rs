mod bullet;
mod collision;
mod common;
mod enemy;
mod explosion;
mod physics;
mod player;
mod ui;

use crate::bullet::*;
use crate::collision::CollisionPlugin;
use crate::common::*;
use crate::enemy::*;
use crate::explosion::*;
use crate::physics::*;
use crate::player::*;
use crate::ui::*;
use bevy::{asset::AssetPath, prelude::*, sprite::collide_aabb::collide};

const PLAYER_SPRITE: &str = "player.png";
const ENEMY_SPRITE: &str = "enemy.png";
const EXPLOSION_SHEET: &str = "explosion.png";

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "shooting".to_string(),
            width: 480.,
            height: 640.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(GameUiPlugin)
        .add_system_to_stage(CoreStage::PreUpdate, handle_input)
        .add_plugin(CollisionPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
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
        player: asset_server.load(PLAYER_SPRITE).into(),
        enemy: asset_server.load(ENEMY_SPRITE).into(),
        explosion: texture_atlases.add(texture_atlas),
    });
}

fn handle_input(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut FireBulletInfo), With<Player>>,
) {
    let (mut velocity, mut fires_bullet) = match player_query.get_single_mut() {
        Ok(x) => x,
        Err(_) => return,
    };

    velocity.dir = Vec2::new(0.0, 0.0);

    if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
        velocity.dir.y += 1.0;
    }

    if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
        velocity.dir.y += -1.0;
    }

    if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
        velocity.dir.x += 1.0;
    }

    if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
        velocity.dir.x += -1.0;
    }

    fires_bullet.can_fire = input.pressed(KeyCode::Space);
}

fn play_se<'a, T: Into<AssetPath<'a>>>(
    asset_server: &Res<AssetServer>,
    audio: &Res<Audio>,
    path: T,
) {
    audio.play_with_settings(
        asset_server.load(path),
        PlaybackSettings::ONCE.with_volume(1.0),
    );
}
