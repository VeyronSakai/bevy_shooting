use bevy::prelude::*;

pub struct FireBulletInfo {
    pub can_fire: bool,
    pub time: f32,
    pub interval: f32,
}

impl FireBulletInfo {
    pub fn is_in_interval(&self) -> bool {
        return self.time < self.interval;
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
