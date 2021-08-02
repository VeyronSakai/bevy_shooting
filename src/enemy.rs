use crate::common::{Materials, WindowSize};
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("enemy_setup", SystemStage::single(spawn_enemy.system()));
    }
}

pub struct Enemy;

fn spawn_enemy(mut commands: Commands, materials: Res<Materials>, window_size: Res<WindowSize>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.enemy.clone(),
            transform: Transform {
                translation: Vec3::new(0., window_size.h / 4., 0.),
                rotation: Quat::from_rotation_z(PI),
                scale: Vec3::new(2., 2., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy);
}
