use crate::common::*;
use crate::physics::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("player_setup", SystemStage::single(spawn_player.system()))
            .add_system(update_player_pos.system());
    }
}

const SPEED: f32 = 20.0;

pub struct Player;

fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(2., 2., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity {
            dir: Vec2::new(0.0, 0.0),
            speed: SPEED,
        })
        .insert(FireBulletInfo {
            can_fire: false,
            time: 0.0,
            interval: 0.1,
        });
}

fn update_player_pos(mut player_query: Query<(&mut Transform, &Velocity), With<Player>>) {
    if let Ok((mut transform, velocity)) = player_query.single_mut() {
        transform.translation.x += velocity.dir.x * velocity.speed * 0.2;
        transform.translation.y += velocity.dir.y * velocity.speed * 0.2;
    }
}
