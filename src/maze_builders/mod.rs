mod aldous_broder;
mod binary_tree;
mod sidewinder;
mod wilson;

pub use aldous_broder::*;
pub use binary_tree::*;
pub use sidewinder::*;
pub use wilson::*;

use bevy::prelude::*;

fn get_direction_from_coinflip(coinflip: bool) -> Dir2 {
    match coinflip {
        true => Dir2::NORTH,
        false => Dir2::EAST,
    }
}
