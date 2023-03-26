use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, window::ExitCondition};
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
            primary_window: Some(Window {
                resolution: (1000., 1000.).into(),
                title: "Snake Game".to_string(),
                ..default()
            }),
            exit_condition: ExitCondition::OnAllClosed,
            close_when_requested: true,
        }))
        .insert_resource(snake::Segments::default())
        .insert_resource(snake::LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameEndEvent>()
        .add_startup_system(setup_camera)
        .add_startup_system(snake::spawn_system)
        .add_system(food::spawn_system.run_if(on_timer(Duration::from_secs_f32(1.0))))
        .add_system(snake::movement_system.run_if(on_timer(Duration::from_secs_f32(0.15))))
        .add_system(
            snake::eating_system
                .after(snake::movement_system)
                .run_if(on_timer(Duration::from_secs_f32(0.15))),
        )
        .add_system(
            snake::growth_system
                .after(snake::eating_system)
                .run_if(on_timer(Duration::from_secs_f32(0.15))),
        )
        .add_system(snake::movement_input_system.before(snake::movement_system))
        .add_system(game::game_over_system.after(snake::movement_system))
        .add_systems(
            (grid::position_translation, grid::size_scaling).in_base_set(CoreSet::PostUpdate),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
