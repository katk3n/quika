use bevy::prelude::*;

#[derive(Component)]
pub struct Revolving {
    pub radius: f32,
    pub threshold: f32,
}
