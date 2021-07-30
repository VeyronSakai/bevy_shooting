mod player;

use bevy::prelude::*;
use crate::player::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "shooting".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system_to_stage(CoreStage::PreUpdate, handle_input.system())
        .run();
}

fn setup(mut commands: Commands) {
    // カメラを生成する
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // 背景の色を黒くする
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

    spawn_player(&mut commands);
}

fn handle_input(input: Res<Input<KeyCode>>, mut player_query: Query<(&mut Velocity), With<Player>>) {
    if let Ok(mut velocity) = player_query.single_mut() {
        velocity.val = Vec2::new(0.0, 0.0);

        if input.pressed(KeyCode::W) {
            velocity.val.y += 1.0;
        }

        if input.pressed(KeyCode::S) {
            velocity.val.y += -1.0;
        }

        if input.pressed(KeyCode::D) {
            velocity.val.x += 1.0;
        }

        if input.pressed(KeyCode::A) {
            velocity.val.x += -1.0;
        }

        println!("{} {}", velocity.val.x, velocity.val.y);
    }
}