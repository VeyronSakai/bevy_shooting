mod player;
mod bullet;
mod physics;

use bevy::prelude::*;
use crate::player::*;
use crate::physics::*;
use crate::bullet::*;

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
        .run();
}

fn setup(mut commands: Commands) {
    // カメラを生成する
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // 背景の色を黒くする
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

    spawn_player(&mut commands);
    spawn_bullet(&mut commands);
}

fn handle_input(input: Res<Input<KeyCode>>, mut player_query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut velocity) = player_query.single_mut() {
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
    }
}