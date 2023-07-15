use bevy::prelude::*;

use self::systems::*;

pub mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, rotate);
    }
}
