mod bullet;
mod common;
mod physics;
mod player;

use crate::bullet::*;
use crate::common::*;
use crate::physics::*;
use crate::player::*;
use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "shooting".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system_to_stage(CoreStage::PreUpdate, handle_input.system())
        .add_system(update_player_pos.system())
        .add_system(update_bullet_pos.system())
        .add_system(spawn_bullet.system())
        .add_system(despawn_bullet.system())
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    // カメラを生成する
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // 背景の色を黒くする
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

    commands.insert_resource(WindowSize {
        w: window.width(),
        h: window.height(),
    });

    spawn_player(&mut commands);
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
