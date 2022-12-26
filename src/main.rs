use bevy::{prelude::*, time::FixedTimestep};
use components::GameEndEvent;
use snake::GrowthEvent;

pub mod components;
pub mod food;
pub mod game;
pub mod grid;
pub mod snake;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Snake Game".to_string(),
                width: 1000.0,
                height: 1000.0,
                ..default()
            },
            ..default()
        }))
        .insert_resource(snake::Segments::default())
        .insert_resource(snake::LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameEndEvent>()
        .add_startup_system(setup_camera)
        .add_startup_system(snake::spawn_system)
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
        .add_system(game::game_over_system.after(snake::movement_system))
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(grid::position_translation)
                .with_system(grid::size_scaling),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
