use bevy::prelude::*;

#[derive(Component)]
pub struct Revolving {
    pub radius: f32,
    pub threshold: f32,
}

#[derive(Component)]
pub struct Bouncing {
    pub threshold: f32,
}
