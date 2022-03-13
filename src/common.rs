use bevy::prelude::*;

#[derive(Component)]
pub struct FireBulletInfo {
    pub can_fire: bool,
    pub time: f32,
    pub duration: f32,
}

impl FireBulletInfo {
    pub fn is_under_suspension(&self) -> bool {
        return self.time < self.duration;
    }
}

#[derive(Component)]
pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Component)]
pub struct Materials {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
}
