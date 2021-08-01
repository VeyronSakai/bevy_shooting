use bevy::prelude::*;

use crate::common::Materials;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("enemy_setup", SystemStage::single(spawn_enemy.system()));
    }
}

struct Enemy;

fn spawn_enemy(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.enemy.clone(),
            sprite: Sprite{
                size: Vec2::new(100.0, 100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy);
}
