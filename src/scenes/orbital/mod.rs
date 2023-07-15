use bevy::prelude::*;

use self::systems::*;

pub mod components;
pub mod systems;

pub struct OrbitalPlugin;

impl Plugin for OrbitalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_systems(Update, (revolve_spheres, bounce_spheres, switch_visibility));
    }
}
