use bevy::prelude::*;

use self::systems::*;

pub mod components;
pub mod systems;

pub struct RipplePlugin;

impl Plugin for RipplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene).add_systems(
            Update,
            (
                bounce_particles,
                spawn_ripple,
                despawn_ripples,
                switch_visibility,
            ),
        );
    }
}
