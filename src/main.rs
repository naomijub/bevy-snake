use bevy::prelude::*;

mod snake;

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(snake::spawn_system)
        .add_plugins(DefaultPlugins)
        .add_system(snake::movement_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
