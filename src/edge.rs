use macroquad::{
    color::*,
    prelude::{
        Vec2, Color,
        draw_line
    }
};
use crate::node::Node;


#[derive(Debug, Clone)]
pub struct Edge {
    pub(crate) id: u32,
    pub(crate) p1: Vec2,
    pub(crate) p2: Vec2,
    pub(crate) col: Color,
    pub(crate) thickness: f32,
    pub(crate) to_n_id: u32,
    pub(crate) from_n_id: u32,
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            id: u32::MAX,
            p1: Vec2::NAN,
            p2: Vec2::NAN,
            col: BLACK,
            thickness: 5.0,
            to_n_id: u32::MAX,
            from_n_id: u32::MAX
        }
    }
}


impl Edge {
    pub fn new(n1: &mut Node, n2: &mut Node) -> Self {
        let edge = Self {
            p1: n1.p,
            p2: n2.p,
            from_n_id: n1.id,
            to_n_id: n2.id,
            ..Edge::default()
        };
        n1.to_e_id.push(edge.id);
        n2.from_e_id.push(edge.id);
        edge
    }

    pub fn set_pos(&mut self, p1: &Vec2, p2: &Vec2) {
        self.p1 = *p1;
        self.p2 = *p2;
    }

    pub fn get_pos(&self) -> (&Vec2, &Vec2) {
        (&self.p1, &self.p2)
    }

    pub fn get_id(&self) -> (u32, u32) {
        (self.from_n_id, self.to_n_id)
    }

    pub fn draw(&self) {
        draw_line(self.p1.x, self.p1.y, self.p2.x, self.p2.y, self.thickness, self.col);
    }



}

