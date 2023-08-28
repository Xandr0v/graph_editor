use crate::{NodePos, NodeKeys, NodeKey};
use serde::{Deserialize, Serialize};
pub use std::collections::HashSet;
use macroquad::{
    color::*,
    prelude::{
        draw_circle,
    }
};
use macroquad::math::Vec2;



pub trait NodeGetSet {
    fn set_p(&mut self, x: f32, y: f32);
    fn set_p_t(&mut self, t: (f32, f32));
    fn set_p_v(&mut self, v: &Vec2);

    fn get_p_t(&self) -> (f32, f32);
    fn get_p_v(&self) -> Vec2;

    fn get_keys(&self) -> &NodeKeys;
    fn get_mut_keys(&mut self) -> &mut NodeKeys;

}

pub(crate) trait NodeGraph {
    fn graph_init(&mut self, key: NodeKey);
    fn pos_changeable(&self);
    fn initialized(&self);
    fn set_p_uns(&mut self, x: f32, y: f32);
    fn set_p_t_uns(&mut self, t: (f32, f32));
    fn set_p_v_uns(&mut self, v: &Vec2);
}



#[derive(Serialize, Deserialize)]
pub struct Node {
    pub(crate) pos: NodePos,
    pub(crate) keys: Option<NodeKeys>,

}

impl Default for Node {
    fn default() -> Self {
        Self {
            pos: NodePos::default(),
            keys: None
        }
    }
}

impl From<(f32, f32)> for Node {
    fn from(p: (f32, f32)) -> Self {
        Self {
            pos: NodePos::new(p.0, p.1),
            ..Node::default()
        }
    }
}

impl From<Vec2> for Node {
    fn from(v: Vec2) -> Self {
        Self {
            pos: NodePos::new(v.x, v.y),
            ..Node::default()
        }
    }
}


impl Node {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: NodePos::new(x, y),
            ..Node::default()
        }
    }

    pub fn draw(&self, r: f32, col: Color) {
        draw_circle(self.pos.x, self.pos.y, r, col);
    }
}


impl NodeGetSet for Node {
    fn set_p(&mut self, x: f32, y: f32) {
        self.pos_changeable();
        self.set_p_uns(x, y);
    }

    fn set_p_t(&mut self, t: (f32, f32)) {
        self.pos_changeable();
        self.set_p_t_uns(t);
    }

    fn set_p_v(&mut self, v: &Vec2) {
        self.pos_changeable();
        self.set_p_v_uns(v);
    }

    fn get_p_t(&self) -> (f32, f32) {
        (self.pos.x, self.pos.y)
    }

    fn get_p_v(&self) -> Vec2 {
        Vec2::new(self.pos.x, self.pos.y)
    }

    fn get_keys(&self) -> &NodeKeys {
        self.keys.as_ref().expect("Node is not added to graph")
    }

    fn get_mut_keys(&mut self) -> &mut NodeKeys {
        self.keys.as_mut().expect("Node is not added to graph")
    }
}



impl NodeGraph for Node {
    fn graph_init(&mut self, key: NodeKey) {
        match self.keys {
            Some(_) => panic!("Node has already been added!"),
            None => self.keys = Some(NodeKeys::new(key))
        }
    }

    fn pos_changeable(&self) {
        if self.keys.is_some() {
            panic!("Direct change of node position is available only when it is not added to graph");
        }
    }

    fn initialized(&self) {
        if self.keys.is_none() {
            panic!("node is not initialized");
        }
    }

    fn set_p_uns(&mut self, x: f32, y: f32) {
        self.pos.x = x; self.pos.y = y;
    }

    fn set_p_t_uns(&mut self, t: (f32, f32)) {
        self.pos.x = t.0; self.pos.y = t.1;
    }

    fn set_p_v_uns(&mut self, v: &Vec2) {
        self.pos.x = v.x; self.pos.y = v.y;
    }
}