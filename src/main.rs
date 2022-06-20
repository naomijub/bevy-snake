use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_plugin(HelloPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn hello_plugin() {
    println!("hello plugin!");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hello_plugin);
    }
}