use bevy::prelude::*;

use crate::components::{Position, Size};

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
pub struct Head;

pub fn spawn_system(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Head)
        .insert(Position { x: 3, y: 3 }) // <--
        .insert(Size::square(0.8)); 
}

#[allow(clippy::needless_pass_by_value)]
pub fn movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<Head>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 1.;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn entity_has_snake_head() {
        // Setup app
        let mut app = App::new();

        // Add startup system
        app.add_startup_system(spawn_system);

        // Run systems
        app.update();

        let mut query = app.world.query_filtered::<Entity, With<Head>>();
        assert_eq!(query.iter(&app.world).count(), 1);
    }

    #[test]
    fn snake_head_has_moved_up() {
        // Setup
        let mut app = App::new();
        let default_transform = Transform { ..default() };

        // Add systems
        app.add_startup_system(spawn_system)
            .add_system(movement_system);

        // Add input resource
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W);
        app.insert_resource(input);

        // Run systems
        app.update();

        let mut query = app.world.query::<(&Head, &Transform)>();
        query.iter(&app.world).for_each(|(_head, transform)| {
            assert!(default_transform.translation.y < transform.translation.y);
            assert_eq!(default_transform.translation.x, transform.translation.x);
        })
    }

    #[test]
    fn snake_head_moves_up_and_right() {
        // Setup
        let mut app = App::new();
        let default_transform = Transform { ..default() };

        // Add systems
        app.add_startup_system(spawn_system)
            .add_system(movement_system);

        // Move Up
        let mut up_transform = Transform { ..default() };
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W);
        app.insert_resource(input);
        app.update();
        let mut query = app.world.query::<(&Head, &Transform)>();
        query.iter(&app.world).for_each(|(_head, transform)| {
            assert!(default_transform.translation.y < transform.translation.y);
            assert_eq!(default_transform.translation.x, transform.translation.x);
            up_transform = transform.to_owned();
        });

        // Move Right
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::D);
        app.insert_resource(input);
        app.update();
        let mut query = app.world.query::<(&Head, &Transform)>();
        query.iter(&app.world).for_each(|(_head, transform)| {
            assert_eq!(up_transform.translation.y, transform.translation.y);
            assert!(up_transform.translation.x < transform.translation.x);
        })
    }

    #[test]
    fn snake_head_moves_down_and_left() {
        // Setup
        let mut app = App::new();
        let default_transform = Transform { ..default() };

        // Add systems
        app.add_startup_system(spawn_system)
            .add_system(movement_system);

        // Move down
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::S);
        app.insert_resource(input);
        app.update();

        // Move Left
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::A);
        app.insert_resource(input);
        app.update();

        // Assert
        let mut query = app.world.query::<(&Head, &Transform)>();
        query.iter(&app.world).for_each(|(_head, transform)| {
            assert!(default_transform.translation.y > transform.translation.y);
            assert!(default_transform.translation.x > transform.translation.x);
        })
    }
}
