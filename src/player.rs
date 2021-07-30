use bevy::ecs::prelude::*;
use bevy::prelude::*;

struct Player {}

pub fn spawn_player(commands: &mut Commands) {
    let sprite_size = Vec2::new(100.0, 100.0);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            size: sprite_size,
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player {});
}