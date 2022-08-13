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
    mut head_positions: Query<&mut Position, With<Head>>,
) {
    for mut position in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::D) {
            position.x += 1;
        }
        if keyboard_input.pressed(KeyCode::W) {
            position.y += 1;
        }
        if keyboard_input.pressed(KeyCode::A) {
            position.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::S) {
            position.y -= 1;
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
        let default_position = Position { x: 3, y: 4 };

        // Add systems
        app.add_startup_system(spawn_system)
            .add_system(movement_system);

        // Add input resource
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W);
        app.insert_resource(input);

        // Run systems
        app.update();

        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(_head, position)| {
            assert_eq!(&default_position, position);
        })
    }

    #[test]
    fn snake_head_moves_up_and_right() {
        // Setup
        let mut app = App::new();
        let up_position = Position { x: 3, y: 4 };

        // Add systems
        app.add_startup_system(spawn_system)
            .add_system(movement_system);

        // Move Up
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W);
        app.insert_resource(input);
        app.update();

        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(_head, position)| {
            assert_eq!(position, &up_position);
        });

        let up_right_position = Position { x: 4, y: 4 };

        // Move Right
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::D);
        app.insert_resource(input);
        app.update();

        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(_head, position)| {
            assert_eq!(&up_right_position, position);
        })
    }

    #[test]
    fn snake_head_moves_down_and_left() {
        // Setup
        let mut app = App::new();
        let down_left_position = Position { x: 2, y: 2 };

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
        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(_head, position)| {
            assert_eq!(&down_left_position, position);
        })
    }
}
