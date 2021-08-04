use bevy::prelude::*;

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

pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

pub struct Materials {
    pub player: Handle<ColorMaterial>,
    pub enemy: Handle<ColorMaterial>,
    pub explosion: Handle<TextureAtlas>,
}
