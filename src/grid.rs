use crate::components::{Position, Size};
use bevy::{prelude::*, window::PrimaryWindow};

#[cfg(debug_assertions)]
pub(crate) const GRID_WIDTH: u16 = 10;
#[cfg(not(debug_assertions))]
pub(crate) const GRID_WIDTH: u16 = 20;
#[cfg(debug_assertions)]
pub(crate) const GRID_HEIGHT: u16 = 10;
#[cfg(not(debug_assertions))]
pub(crate) const GRID_HEIGHT: u16 = 20;

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::needless_pass_by_value)]
pub fn size_scaling(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = primary_window.get_single().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        scale_sprite(transform.as_mut(), sprite_size, window);
    }
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::needless_pass_by_value)]
pub fn position_translation(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    let window = primary_window.get_single().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        translate_position(transform.as_mut(), pos, window);
    }
}

#[allow(clippy::cast_lossless)]
fn scale_sprite(transform: &mut Transform, sprite_size: &Size, window: &Window) {
    transform.scale = Vec3::new(
        sprite_size.width / GRID_WIDTH as f32 * window.width(),
        sprite_size.height / GRID_HEIGHT as f32 * window.height(),
        1.0,
    );
}

#[allow(clippy::cast_lossless)]
fn convert(pos: f32, bound_window: f32, grid_side_lenght: f32) -> f32 {
    let tile_size = bound_window / grid_side_lenght;
    (pos / grid_side_lenght).mul_add(bound_window, -bound_window / 2.) + (tile_size / 2.)
}

#[allow(clippy::cast_lossless)]
fn translate_position(transform: &mut Transform, pos: &Position, window: &Window) {
    transform.translation = Vec3::new(
        convert(pos.x as f32, window.width(), GRID_WIDTH as f32),
        convert(pos.y as f32, window.height(), GRID_HEIGHT as f32),
        0.0,
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::Size;
    use approx::assert_relative_eq;
    use bevy::window::WindowResolution;

    #[test]
    fn transform_has_correct_scale_for_window() {
        // Setup
        #[cfg(debug_assertions)]
        let expected_transform = Transform {
            scale: Vec3::new(20., 20., 1.),
            ..default()
        };
        #[cfg(not(debug_assertions))]
        let expected_transform = Transform {
            scale: Vec3::new(10., 10., 1.),
            ..default()
        };

        let mut default_transform = Transform {
            scale: Vec3::new(2., 3., 4.),
            ..default()
        };
        let sprite_size = Size::square(1.);

        // Create window
        let window = Window {
            resolution: WindowResolution::new(200., 200.),
            ..default()
        };

        // Apply scale
        scale_sprite(&mut default_transform, &sprite_size, &window);

        assert_eq!(default_transform, expected_transform);
    }

    #[test]
    fn convert_position_x_for_grid_width() {
        let x = convert(4., 400., GRID_WIDTH as f32);

        #[cfg(debug_assertions)]
        assert_relative_eq!(x, -20., epsilon = 0.00001);
        #[cfg(not(debug_assertions))]
        assert_relative_eq!(x, -110., epsilon = 0.00001);
    }

    #[test]
    fn convert_position_y_for_grid_height() {
        let x = convert(5., 400., GRID_HEIGHT as f32);

        #[cfg(debug_assertions)]
        assert_relative_eq!(x, 20., epsilon = 0.00001);
        #[cfg(not(debug_assertions))]
        assert_relative_eq!(x, -90., epsilon = 0.00001)
    }

    #[test]
    fn translate_position_to_window() {
        let position = Position { x: 2, y: 8 };
        let mut default_transform = Transform::default();
        let expected = Transform {
            #[cfg(debug_assertions)]
            translation: Vec3::new(-100., 140., 0.),
            #[cfg(not(debug_assertions))]
            translation: Vec3::new(-150., -29.999996, 0.),
            ..default()
        };

        // Create window
        // let mut descriptor = Window::default();
        // descriptor.
        // descriptor.height = 400.;
        // descriptor.width = 400.;
        // let window = Window::new(WindowId::new(), &descriptor, 400, 400, 1., None, None);
        let window = Window {
            resolution: WindowResolution::new(400., 400.),
            ..default()
        };

        // Apply translation
        translate_position(&mut default_transform, &position, &window);

        assert_eq!(default_transform, expected);
    }
}
