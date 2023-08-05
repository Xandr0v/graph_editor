use macroquad::{
    color::*,
    prelude::{
        Vec2, Color,
        draw_circle,
    }
};
use crate::{EdgeKey, NodeKey};


impl From<&Vec2> for Node {
    fn from(p: &Vec2) -> Self {
        Self {
            p: *p,
            ..Node::default()
        }
    }
}


impl From<(f32, f32)> for Node {
    fn from(p: (f32, f32)) -> Self {
        Self {
            p: Vec2::new(p.0, p.1),
            ..Node::default()
        }
    }
}



pub struct NodeId {
    pub own: NodeKey,
    pub heads: Vec<EdgeKey>,
    pub tails: Vec<EdgeKey>
}

impl NodeId {
    pub fn new(own: NodeKey) -> Self {
        Self {
            own,
            heads: vec![],
            tails: vec![]
        }
    }
}



pub struct Node {
    pub(crate) p: Vec2,
    pub(crate) col: Color,
    pub(crate) rad: f32,
    pub id: Option<NodeId>,

}

impl Default for Node {
    fn default() -> Self {
        Self {
            p: Vec2::default(),
            col: Color::new(0.0, 0.0, 0.0, 0.5),
            rad: 20.0,
            id: None,
        }
    }
}

impl Node {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            p: Vec2::new(x, y),
            ..Node::default()
        }
    }


    pub fn get_mut_id(&mut self) -> &mut NodeId {
        self.id.as_mut().expect("Node is not added to graph")
    }

    pub fn get_id(&self) -> &NodeId {
        self.id.as_ref().expect("Node is not added to graph")
    }



    pub fn get_pos(&self) -> &Vec2 {
        &self.p
    }

    pub fn draw(&self) {
        draw_circle(self.p.x, self.p.y, self.rad, self.col);
    }

    pub fn draw_selected(&self) {
        draw_circle(self.p.x, self.p.y, self.rad * 1.2, RED);
    }
}

