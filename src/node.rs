use macroquad::{
    color::*,
    prelude::{
        Vec2, Color,
        draw_circle,
    }
};

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) id: u32,
    pub(crate) p: Vec2,
    pub(crate) col: Color,
    pub(crate) rad: f32,
    pub(crate) to_e_id: Vec<u32>,
    pub(crate) from_e_id: Vec<u32>
}


impl From<Vec2> for Node {
    fn from(p: Vec2) -> Self {
        Self {
            p,
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

impl Default for Node {
    fn default() -> Self {
        Self {
            id: u32::MAX,
            p: Vec2::NAN,
            col: BLACK,
            rad: 10.0,
            to_e_id: vec![],
            from_e_id: vec![],
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

    pub fn set_pos(&mut self, p: Vec2) {
        self.p = p;
    }

    pub fn get_pos(&self) -> Vec2 {
        self.p
    }

    pub fn draw(&self) {
        draw_circle(self.p.x, self.p.y, self.rad, self.col);
    }

    pub fn draw_selected(&self) {
        draw_circle(self.p.x, self.p.y, self.rad * 1.2, RED);
    }
}

