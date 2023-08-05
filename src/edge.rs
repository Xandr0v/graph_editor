use macroquad::{
    color::*,
    prelude::{
        Vec2, Color,
        draw_line
    }
};
use crate::{EdgeKey, NodeKey};

pub struct EdgeId {
    pub own: EdgeKey,
    pub from: NodeKey,
    pub to: NodeKey,
}

impl EdgeId {
    pub fn new(own: EdgeKey, from: NodeKey, to: NodeKey) -> Self {
        Self {
            own,
            from,
            to
        }
    }
}


pub struct Edge {
    pub(crate) p1: Vec2,
    pub(crate) p2: Vec2,
    pub(crate) col: Color,
    pub(crate) thickness: f32,
    pub(crate) id: Option<EdgeId>
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            p1: Vec2::default(),
            p2: Vec2::default(),
            col: Color::new(0.0, 0.0, 0.0, 0.5),
            thickness: 5.0,
            id: None
        }
    }
}


impl Edge {
    pub fn new(p1: &Vec2, p2: &Vec2) -> Self {
        Self {
            p1: *p1,
            p2: *p2,
            ..Edge::default()
        }
    }

    pub fn get_pos(&self) -> (&Vec2, &Vec2) {
        (&self.p1, &self.p2)
    }

    pub fn draw(&self) {
        draw_line(self.p1.x, self.p1.y, self.p2.x, self.p2.y, self.thickness, self.col);
    }

}

