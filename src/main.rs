use bevy::prelude::*;

pub mod components;
pub mod grid;
pub mod snake;


fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(snake::spawn_system)
        .add_plugins(DefaultPlugins)
        .add_system(snake::movement_system)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(grid::position_translation)
                .with_system(grid::size_scaling),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
