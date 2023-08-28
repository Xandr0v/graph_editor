pub use macroquad::{
    window::*,
    input::*,
    color::*,
};



mod edge_src; pub use edge_src::*;
pub use edge_src::edge::*;
pub use edge_src::edge_pos::*;
pub use edge_src::edge_keys::*;

mod node_src;
pub use node_src::node::*;
pub use node_src::node_pos::*;
pub use node_src::node_keys::*;

mod path_algo; pub use path_algo::*;
mod graph; pub use graph::*;
mod variables; pub use variables::*;


