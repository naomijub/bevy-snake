use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
pub struct SnakeHead;

pub fn spawn_snake(mut commands: Commands) {
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
        .insert(SnakeHead);
}

pub fn snake_movement(mut head_positions: Query<(&SnakeHead, &mut Transform)>) {
    for (_head, mut transform) in head_positions.iter_mut() {
        transform.translation.y += 1.;
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
        app.add_startup_system(spawn_snake);

        // Run systems
        app.update();

        let mut query = app.world.query_filtered::<Entity, With<SnakeHead>>();
        assert_eq!( query.iter(&app.world).count(), 1);
    }

    #[test]
    fn snake_head_has_moved_up() {
        // Setup
        let mut app = App::new();
        let default_transform = Transform {..default()};

        // Add systems
        app.add_startup_system(spawn_snake)
        .add_system(snake_movement);

        // Add input resource
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W);
        app.insert_resource(input);

        // Run systems
        app.update();

        let mut query = app.world.query::<(&SnakeHead, &Transform)>();
        query.iter(&app.world).for_each(|(_head, transform)| {
            assert!(default_transform.translation.y < transform.translation.y);
            assert_eq!(default_transform.translation.x, transform.translation.x);
        })
    }
}