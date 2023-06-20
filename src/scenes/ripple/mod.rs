use bevy::prelude::*;

use self::systems::*;

pub mod components;
pub mod systems;

pub struct RipplePlugin;

impl Plugin for RipplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_scene)
            .add_system(bounce_particles)
            .add_system(spawn_ripple)
            .add_system(despawn_ripples)
            .add_system(switch_visibility);
    }
}
