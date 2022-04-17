use bevy::prelude::*;

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionTimer {
    pub value: Timer,
}

pub fn animate_explosion_sprite(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            Entity,
            &mut ExplosionTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Explosion>,
    >,
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.value.tick(time.delta());
        if timer.value.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index += 1;
            if sprite.index == texture_atlas.textures.len() {
                commands.entity(entity).despawn()
            }
        }
    }
}
