use std::fmt::{self, Display};

use bevy::prelude::Component;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

#[derive(Component, Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    #[must_use]
    pub const fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum GameEndEvent {
    GameOver,
}

impl Default for GameEndEvent {
    fn default() -> Self {
        Self::GameOver
    }
}

impl Display for GameEndEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameEndEvent::GameOver => write!(f, "Game Over!"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sized_square_is_created_calling_square_fn() {
        let expected = Size {
            width: 3.14,
            height: 3.14,
        };
        let actual = Size::square(3.14);

        assert_eq!(actual, expected);
    }

    #[test]
    fn opposite_direction() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Right.opposite(), Direction::Left);
        assert_eq!(Direction::Left.opposite(), Direction::Right);
    }
}
