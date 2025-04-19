use bevy::color::palettes::css::*;
use bevy::prelude::*;

pub fn get_cell_background_color(
    distance: Option<&i32>,
    farthest_distance: i32,
    has_overlay: bool,
) -> Color {
    if !has_overlay {
        Color::srgba(0.8, 0.8, 0.8, 1.0)
    } else if let Some(distance) = distance {
        let max = farthest_distance as f32;
        let dist = *distance as f32;
        let intensity = (max - dist) / max;
        let dark = 1.0 * intensity;
        let bright = 0.5 + (0.5 * intensity);
        Color::srgb(dark, bright, dark)
    } else {
        Color::srgba(1.0, 1.0, 1.0, 0.0)
    }
}

pub fn get_cell_text(
    is_start: bool,
    is_end: bool,
    distance: Option<&i32>,
    has_overlay: bool,
) -> String {
    if !has_overlay {
        if is_start {
            return String::from("GO");
        } else if is_end {
            return String::from("END");
        }
    } else if let Some(distance) = distance {
        return format!("{distance}");
    }

    String::from("")
}

pub fn get_cell_text_color(is_on_path: bool) -> Color {
    if is_on_path {
        Color::Srgba(RED)
    } else {
        Color::Srgba(BLACK)
    }
}
