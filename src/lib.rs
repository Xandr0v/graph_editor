pub use macroquad::{
    window::*,
    math::*,
    color::*,
    input::*
};
pub use itertools::Itertools;

mod node; pub use node::*;
mod edge; pub use edge::*;
mod graph; pub use graph::*;

