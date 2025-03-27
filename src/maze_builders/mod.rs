mod binary_tree;
mod sidewinder;

pub use binary_tree::*;
pub use sidewinder::*;

use bevy::prelude::*;

fn get_direction_from_coinflip(coinflip: bool) -> Dir2 {
    match coinflip {
        true => Dir2::NORTH,
        false => Dir2::EAST,
    }
}
