use bevy::prelude::*;

use self::systems::*;

pub mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(rotate);
    }
}
