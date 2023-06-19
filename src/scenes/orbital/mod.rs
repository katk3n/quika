use bevy::prelude::*;

use self::systems::*;

pub mod components;
pub mod systems;

pub struct OrbitalPlugin;

impl Plugin for OrbitalPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_scene)
            .add_system(revolve_spheres)
            .add_system(bounce_spheres);
    }
}
