use bevy::prelude::*;

#[derive(Component)]
pub struct Bouncing;

#[derive(Component)]
pub struct Source {
    pub point: Vec3,
    pub spawn_time: f32,
    pub magnitude: f32,
}
