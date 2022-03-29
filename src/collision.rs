use bevy::prelude::Plugin;
use crate::{App, animate_explosion_sprite, enemy_bullet_collide_player, player_bullet_collide_enemy};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_explosion_sprite)
            .add_system(player_bullet_collide_enemy)
            .add_system(enemy_bullet_collide_player);
    }
}