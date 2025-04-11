use bevy::prelude::*;
use camera::CameraPlugin;
use startrace_lab::StartracePlugin;

mod camera;
mod magmagrid_lab;
mod startrace_lab;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(StartracePlugin)
        .run();
}
