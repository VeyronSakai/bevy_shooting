use crate::common::*;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("enemy_setup", SystemStage::single(spawn_enemy.system()));
    }
}

#[derive(Component)]
pub struct Enemy;

fn spawn_enemy(mut commands: Commands, materials: Res<Materials>, window_size: Res<WindowSize>) {
    spawn_enemy_internal(
        &mut commands,
        &materials,
        Vec3::new(window_size.w / 4., window_size.h / 4., 0.),
    );
    spawn_enemy_internal(
        &mut commands,
        &materials,
        Vec3::new(-window_size.w / 4., window_size.h / 4., 0.),
    );
    spawn_enemy_internal(
        &mut commands,
        &materials,
        Vec3::new(-window_size.w / 8., window_size.h / 3., 0.),
    );
    spawn_enemy_internal(
        &mut commands,
        &materials,
        Vec3::new(window_size.w / 8., window_size.h / 3., 0.),
    );
}

fn spawn_enemy_internal(commands: &mut Commands, materials: &Res<Materials>, position: Vec3) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: materials.enemy.clone(),
            transform: Transform {
                translation: position,
                rotation: Quat::from_rotation_z(PI),
                scale: Vec3::new(2., 2., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(FireBulletInfo {
            can_fire: true,
            time: 0.0,
            duration: 1.25,
        });
}
