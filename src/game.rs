use bevy::prelude::*;

use crate::components::GameEndEvent;

pub fn game_over_system(mut commands: Commands, mut reader: EventReader<GameEndEvent>) {
    if reader.iter().next().is_some() {
        commands.spawn_empty().insert(GameEndEvent::GameOver);
        println!("{}", GameEndEvent::GameOver);
    }
}

#[cfg(test)]
pub mod test {

    use super::*;
    use crate::components::Position;
    use crate::snake::{self, Head, LastTailPosition, Segments};

    #[test]
    fn game_end_event_with_game_over() {
        // Setup
        let mut app = App::new();

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_startup_system(snake::spawn_system)
            .add_system(snake::movement_system)
            .add_system(snake::movement_input_system.before(snake::movement_system))
            .add_system(game_over_system.after(snake::movement_system));

        // Add new input resource
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W);
        app.insert_resource(input);

        // Run systems again
        app.update(); // x: 3, y: 4
        app.update(); // x: 3, y: 5
        app.update(); // x: 3, y: 6
        app.update(); // x: 3, y: 7
        app.update(); // x: 3, y: 8
        app.update(); // x: 3, y: 9

        let mut query = app.world.query::<&GameEndEvent>();
        assert_eq!(query.iter(&app.world).count(), 0);

        app.update(); // x: 3, y: 10

        let mut query = app.world.query::<&GameEndEvent>();
        assert_eq!(query.iter(&app.world).count(), 1);

        let mut query = app.world.query_filtered::<&Position, With<Head>>();
        let position_at_gameover = query.iter(&app.world).next().unwrap();
        let snake_position_after_game_over = position_at_gameover.clone();

        app.update();

        let mut query = app.world.query_filtered::<&Position, With<Head>>();
        let position_after_gameover = query.iter(&app.world).next().unwrap();

        assert_eq!(
            snake_position_after_game_over,
            position_after_gameover.clone()
        );
    }

    #[test]
    fn game_end_event_with_game_over_when_moving_left() {
        // Setup
        let mut app = App::new();

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_startup_system(snake::spawn_system)
            .add_system(snake::movement_system)
            .add_system(snake::movement_input_system.before(snake::movement_system))
            .add_system(game_over_system.after(snake::movement_system));

        // Add new input resource
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::A);
        app.insert_resource(input);

        // Run systems again
        app.update(); // x: 2, y: 3
        app.update(); // x: 1, y: 3
        app.update(); // x: 0, y: 3

        let mut query = app.world.query::<&GameEndEvent>();
        assert_eq!(query.iter(&app.world).count(), 0);

        app.update(); // x: -1, y: 3

        let mut query = app.world.query::<&GameEndEvent>();
        assert_eq!(query.iter(&app.world).count(), 1);
    }

    #[test]
    fn game_end_event_with_game_over_when_moving_right() {
        // Setup
        let mut app = App::new();

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_startup_system(snake::spawn_system)
            .add_system(snake::movement_system)
            .add_system(snake::movement_input_system.before(snake::movement_system))
            .add_system(game_over_system.after(snake::movement_system));

        // Add new input resource
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::D);
        app.insert_resource(input);

        // Run systems again
        app.update(); // x: 4, y: 3
        app.update(); // x: 5, y: 3
        app.update(); // x: 6, y: 3
        app.update(); // x: 7, y: 3
        app.update(); // x: 8, y: 3
        app.update(); // x: 9, y: 3
        app.update(); // x: 10, y: 3

        let mut query = app.world.query::<&GameEndEvent>();
        assert_eq!(query.iter(&app.world).count(), 1);
    }
}
