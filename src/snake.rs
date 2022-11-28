use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    components::{Direction, GameEndEvent, Position, Size},
    food::Food,
    grid::{GRID_HEIGHT, GRID_WIDTH},
};

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.8, 0.0, 0.8); // <--

#[derive(Component)]
pub struct Head {
    direction: Direction,
}

#[derive(Component)]
pub struct Segment;

#[derive(Default, Deref, DerefMut)]
pub struct Segments(Vec<Entity>);

pub struct GrowthEvent;

#[derive(Default)]
pub struct LastTailPosition(Option<Position>);

impl Default for Head {
    fn default() -> Self {
        Self {
            direction: Direction::Up,
        }
    }
}

pub fn spawn_system(mut commands: Commands, mut segments: ResMut<Segments>) {
    *segments = Segments(vec![
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
            .insert(Head::default())
            .insert(Segment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment_system(commands, Position { x: 3, y: 2 }),
    ]);
}

pub fn spawn_segment_system(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Segment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

pub fn movement_system(
    segments: ResMut<Segments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_end_writer: EventWriter<GameEndEvent>,
    heads: Query<(Entity, &Head)>,
    mut positions: Query<(Entity, &Segment, &mut Position)>,
    game_end: Query<&GameEndEvent>,
) {
    let positions_clone: HashMap<Entity, Position> = positions
        .iter()
        .map(|(entity, _segment, position)| (entity, position.clone()))
        .collect();
    if let Some((id, head)) = heads.iter().next() {
        (*segments).windows(2).for_each(|entity| {
            if let Ok((_, _segment, mut position)) = positions.get_mut(entity[1]) {
                if let Some(new_position) = positions_clone.get(&entity[0]) {
                    *position = new_position.clone();
                }
            };
        });
        if game_end.is_empty() {
            let _ = positions.get_mut(id).map(|(_, _segment, mut pos)| {
                match &head.direction {
                    Direction::Left => {
                        pos.x -= 1;
                    }
                    Direction::Right => {
                        pos.x += 1;
                    }
                    Direction::Up => {
                        pos.y += 1;
                    }
                    Direction::Down => {
                        pos.y -= 1;
                    }
                };
                if pos.x < 0
                    || pos.y < 0
                    || pos.x as u16 >= GRID_WIDTH
                    || pos.y as u16 >= GRID_HEIGHT
                {
                    game_end_writer.send(GameEndEvent::GameOver);
                }
            });
        }
        *last_tail_position = LastTailPosition(Some(
            positions_clone
                .get(segments.last().unwrap())
                .unwrap()
                .clone(),
        ));
    }
}

pub fn eating_system(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<Head>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn growth_system(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<Segments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment_system(
            commands,
            last_tail_position.0.clone().unwrap(),
        ));
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn movement_input_system(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut Head>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::A) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::S) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::W) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::D) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
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
        app.insert_resource(Segments::default())
            .add_startup_system(spawn_system);

        // Run systems
        app.update();

        let mut query = app.world.query_filtered::<Entity, With<Head>>();
        assert_eq!(query.iter(&app.world).count(), 1);
    }

    #[test]
    fn snake_starts_moviment_up() {
        // Setup app
        let mut app = App::new();

        // Add startup system
        app.insert_resource(Segments::default())
            .add_startup_system(spawn_system);

        // Run systems
        app.update();

        let mut query = app.world.query::<&Head>();
        let head = query.iter(&app.world).next().unwrap();
        assert_eq!(head.direction, Direction::Up);
    }

    #[test]
    fn snake_head_has_moved_up() {
        // Setup
        let mut app = App::new();
        let default_position = Position { x: 3, y: 4 };

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_startup_system(spawn_system)
            .add_system(movement_system)
            .add_system(movement_input_system.before(movement_system));

        // Add input resource
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W);
        app.insert_resource(input);

        // Run systems
        app.update();

        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(head, position)| {
            assert_eq!(&default_position, position);
            assert_eq!(head.direction, Direction::Up);
        })
    }

    #[test]
    fn snake_head_moves_up_and_right() {
        // Setup
        let mut app = App::new();
        let up_position = Position { x: 3, y: 4 };

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_startup_system(spawn_system)
            .add_system(movement_system)
            .add_system(movement_input_system.before(movement_system));

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
        query.iter(&app.world).for_each(|(head, position)| {
            assert_eq!(&up_right_position, position);
            assert_eq!(head.direction, Direction::Right);
        })
    }

    #[test]
    fn snake_head_moves_down_and_left() {
        // Setup
        let mut app = App::new();
        let down_left_position = Position { x: 2, y: 2 };

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_startup_system(spawn_system)
            .add_system(movement_system)
            .add_system(movement_input_system.before(movement_system));

        // Move Left
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::A);
        app.insert_resource(input);
        app.update();

        // Move down
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::S);
        app.insert_resource(input);
        app.update();

        // Assert
        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(head, position)| {
            assert_eq!(&down_left_position, position);
            assert_eq!(head.direction, Direction::Down);
        })
    }

    #[test]
    fn snake_cannot_start_moving_down() {
        // Setup
        let mut app = App::new();
        let down_left_position = Position { x: 3, y: 4 };

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_startup_system(spawn_system)
            .add_system(movement_system)
            .add_system(movement_input_system.before(movement_system));

        // Move down
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::S);
        app.insert_resource(input);
        app.update();

        // Assert
        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(_head, position)| {
            assert_eq!(&down_left_position, position);
        })
    }

    #[test]
    fn entity_snake_has_two_segments() {
        // Setup app
        let mut app = App::new();

        // Add startup system
        app.insert_resource(Segments::default())
            .add_startup_system(spawn_system);

        // Run systems
        app.update();

        let mut query = app.world.query_filtered::<Entity, With<Segment>>();
        assert_eq!(query.iter(&app.world).count(), 2);
    }

    #[test]
    fn snake_segment_has_followed_head() {
        // Setup
        let mut app = App::new();
        let new_position_head_right = Position { x: 4, y: 3 };
        let new_position_segment_right = Position { x: 3, y: 3 };

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>()
            .add_startup_system(spawn_system)
            .add_system(movement_system)
            .add_system(movement_input_system.before(movement_system));

        // Add input resource
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::D);
        app.insert_resource(input);

        // Run systems
        app.update();

        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(head, position)| {
            assert_eq!(&new_position_head_right, position);
            assert_eq!(head.direction, Direction::Right);
        });

        let mut query = app.world.query::<(&Segment, &Position, Without<Head>)>();
        query.iter(&app.world).for_each(|(_segment, position, _)| {
            assert_eq!(&new_position_segment_right, position);
        });

        // New expected positions:
        let new_position_head_up = Position { x: 4, y: 4 };
        let new_position_segment_up = Position { x: 4, y: 3 };

        // Add new input resource
        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::W);
        app.insert_resource(input);

        // Run systems again
        app.update();

        let mut query = app.world.query::<(&Head, &Position)>();
        query.iter(&app.world).for_each(|(head, position)| {
            assert_eq!(&new_position_head_up, position);
            assert_eq!(head.direction, Direction::Up);
        });

        let mut query = app.world.query::<(&Segment, &Position, Without<Head>)>();
        query.iter(&app.world).for_each(|(_segment, position, _)| {
            assert_eq!(&new_position_segment_up, position);
        })
    }

    #[test]
    fn snake_grows_when_eating() {
        // Setup
        let mut app = App::new();

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GrowthEvent>()
            .add_event::<GameEndEvent>()
            .add_startup_system(spawn_system)
            .add_system(crate::food::spawn_system)
            .add_system_set(
                SystemSet::new()
                    .with_system(movement_system)
                    .with_system(eating_system.after(movement_system))
                    .with_system(growth_system.after(eating_system)),
            );

        // Run systems
        app.update();

        let mut query = app.world.query::<(&Segment, &Position)>();
        assert_eq!(query.iter(&app.world).count(), 2);
        let mut query = app.world.query::<(&Food, &Position)>();
        assert_eq!(query.iter(&app.world).count(), 1);

        app.update();

        let mut query = app.world.query::<(&Segment, &Position)>();
        assert_eq!(query.iter(&app.world).count(), 3);
    }
}
