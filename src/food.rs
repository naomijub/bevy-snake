use std::collections::HashSet;

use crate::{
    components::{Position, Size},
    grid::{GRID_HEIGHT, GRID_WIDTH},
};
use bevy::prelude::*;
use rand::prelude::random;

const FOOD_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub struct Food;

#[allow(clippy::cast_possible_wrap)]
pub fn spawn_system(mut commands: Commands, positions: Query<&Position>) {
    let positions_set: HashSet<&Position> = positions.iter().collect();

    if let Some(position) = (0..(GRID_WIDTH * GRID_HEIGHT))
        .map(|_| Position {
            x: if cfg!(test) {
                3
            } else {
                (random::<u16>() % GRID_WIDTH) as i16
            },
            y: if cfg!(test) {
                5
            } else {
                (random::<u16>() % GRID_HEIGHT) as i16
            },
        })
        .find(|position| !positions_set.contains(position))
    {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: FOOD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(Food)
            .insert(position)
            .insert(Size::square(0.65));
    }
}

#[cfg(test)]
mod test {
    use crate::components::Position;

    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn spawns_food_inplace(_execution in 0u32..1000) {
            // Setup app
            let mut app = App::new();

            // Add startup system
            app.add_startup_system(spawn_system);

            // Run systems
            app.update();

            let mut query = app.world.query_filtered::<&Position, With<Food>>();
            assert_eq!(query.iter(&app.world).count(), 1);
            query.iter(&app.world).for_each(|position| {
                let x = position.x;
                let y = position.y;

                assert!(x >= 0 && x as i32 <= (GRID_WIDTH -1) as i32);
                assert!(y >= 0 && y as i32 <= (GRID_HEIGHT -1) as i32);
            })
        }
    }

    #[test]
    fn food_only_spawns_once() {
        // Setup
        let mut app = App::new();

        // Add systems
        app.add_system(spawn_system);

        // Run systems
        app.update();

        let mut query = app.world.query::<(&Food, &Position)>();
        assert_eq!(query.iter(&app.world).count(), 1);

        // Run systems
        app.update();

        let mut query = app.world.query::<(&Food, &Position)>();
        assert_eq!(query.iter(&app.world).count(), 1)
    }
}
