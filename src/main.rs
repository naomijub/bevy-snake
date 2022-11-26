use bevy::{core::FixedTimestep, prelude::*};
use snake::GrowthEvent;

pub mod components;
pub mod food;
pub mod grid;
pub mod snake;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake Game".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        .insert_resource(snake::Segments::default())
        .insert_resource(snake::LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .add_startup_system(setup_camera)
        .add_startup_system(snake::spawn_system)
        .add_plugins(DefaultPlugins)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food::spawn_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake::movement_system)
                .with_system(snake::eating_system.after(snake::movement_system))
                .with_system(snake::growth_system.after(snake::eating_system)),
        )
        .add_system(snake::movement_input_system.before(snake::movement_system))
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
