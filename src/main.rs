use bevy::prelude::*;

mod snake;

use snake::{spawn_snake, snake_movement};

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_plugins(DefaultPlugins)
        .add_system(snake_movement)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}