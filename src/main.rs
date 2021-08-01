mod bullet;
mod common;
mod physics;
mod player;
mod enemy;

use crate::bullet::*;
use crate::common::*;
use crate::physics::*;
use crate::player::*;
use crate::enemy::*;
use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player.png";

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

    let enemy_color = Color::rgb(0.7, 0.7, 0.7);

    commands.insert_resource(Materials {
        player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        enemy: materials.add(enemy_color.into())
    });

    let enemy_mat = materials.add(enemy_color.into());
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
