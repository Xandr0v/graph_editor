use crate::{EdgePos, EdgeKeys, EdgeKey, NodeKey};
use serde::{Deserialize, Serialize};
use macroquad::{
    color::*,
    prelude::{
        draw_line
    }
};
use macroquad::math::Vec2;
use macroquad::shapes::draw_triangle;


pub trait EdgeGetSet {
    fn set_p1(&mut self, x: f32, y: f32);
    fn set_p2(&mut self, x: f32, y: f32);
    fn set_p1_t(&mut self, t: (f32, f32));
    fn set_p2_t(&mut self, t: (f32, f32));
    fn set_p1_v(&mut self, v: &Vec2);
    fn set_p2_v(&mut self, v: &Vec2);

    fn get_p1_t(&self) -> (f32, f32);
    fn get_p2_t(&self) -> (f32, f32);
    fn get_p1_v(&self) -> Vec2;
    fn get_p2_v(&self) -> Vec2;

    fn get_length(&self) -> f32;
    fn get_keys(&self) -> &EdgeKeys;
    fn get_mut_keys(&mut self) -> &mut EdgeKeys;
}

pub(crate) trait EdgeGraph {
    fn graph_init(&mut self, key: EdgeKey, tail_key: NodeKey, head_key: NodeKey);
    fn pos_changeable(&self);
    fn initialized(&self);

    fn set_p1_uns(&mut self, x: f32, y: f32);
    fn set_p2_uns(&mut self, x: f32, y: f32);
    fn set_p1_t_uns(&mut self, t: (f32, f32));
    fn set_p2_t_uns(&mut self, t: (f32, f32));
    fn set_p1_v_uns(&mut self, v: &Vec2);
    fn set_p2_v_uns(&mut self, v: &Vec2);
}




#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub(crate) pos: EdgePos,
    pub keys: Option<EdgeKeys>
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            pos: EdgePos::default(),
            keys: None
        }
    }
}



impl Edge {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            pos: EdgePos::new(x1, y1, x2, y2),
            ..Edge::default()
        }
    }


    pub fn from(p1: Vec2, p2: Vec2) -> Self {
        Self {
            pos: EdgePos::new(p1.x, p1.y, p2.x, p2.y),
            keys: None
        }
    }


    pub fn draw(&self, t: f32, col: Color) {
        let v1 = self.get_p1_v();
        let v2 = self.get_p2_v();
        let v = v2 - v1;
        let vn = v.normalize();
        let vpn = vn.perp();

        let vl1 = v1 + vn*R;
        let vl2 = v2 - vn*(R + AT);
        draw_line(vl1.x, vl1.y, vl2.x, vl2.y, t, col);
        draw_triangle(vl2 + vn* AT, vl2 + vpn* AT, vl2 - vpn* AT, col);

    }



    pub fn draw_selected(&self, t: f32, ts: f32, col: Color) {
        let v1 = self.get_p1_v();
        let v2 = self.get_p2_v();
        let v = v2 - v1;
        let vn = v.normalize();
        let vpn = vn.perp();

        let td = (ts - t)/2.0;
        let ats = AT + 2.41*td;
        let vl1 = v1 + vn*R;
        let vl2 = v2 - vn*(R + AT + td);
        draw_line(vl1.x, vl1.y, vl2.x, vl2.y, ts, col);
        draw_triangle(vl2 + vn*ats, vl2 + vpn*ats, vl2 - vpn*ats, col);
    }

}


const R: f32 = 10.0;
const AT: f32 = 10.0;

impl EdgeGetSet for Edge {
    fn set_p1(&mut self, x: f32, y: f32) {
        self.pos_changeable();
        self.set_p1_uns(x, y);
    }

    fn set_p2(&mut self, x: f32, y: f32) {
        self.pos_changeable();
        self.set_p2_uns(x, y);
    }

    fn set_p1_t(&mut self, t: (f32, f32)) {
        self.pos_changeable();
        self.set_p1_t_uns(t);
    }

    fn set_p2_t(&mut self, t: (f32, f32)) {
        self.pos_changeable();
        self.set_p2_t_uns(t);
    }

    fn set_p1_v(&mut self, v: &Vec2) {
        self.pos_changeable();
        self.set_p1_v_uns(v);
    }

    fn set_p2_v(&mut self, v: &Vec2) {
        self.pos_changeable();
        self.set_p2_v_uns(v);
    }


    fn get_p1_t(&self) -> (f32, f32) {
        (self.pos.x1, self.pos.y1)
    }

    fn get_p2_t(&self) -> (f32, f32) {
        (self.pos.x2, self.pos.y2)
    }

    fn get_p1_v(&self) -> Vec2 {
        Vec2::new(self.pos.x1, self.pos.y1)
    }

    fn get_p2_v(&self) -> Vec2 {
        Vec2::new(self.pos.x2, self.pos.y2)
    }

    fn get_length(&self) -> f32 {
        Vec2::distance(self.get_p1_v(), self.get_p2_v())
    }

    fn get_keys(&self) -> &EdgeKeys {
        self.keys.as_ref().expect("Edge is not added to graph")
    }

    fn get_mut_keys(&mut self) -> &mut EdgeKeys {
        self.keys.as_mut().expect("Edge is not added to graph")
    }
}

impl EdgeGraph for Edge {
    fn graph_init(&mut self, key: EdgeKey, tail_key: NodeKey, head_key: NodeKey) {
        match self.keys {
            Some(_) => panic!("Edge has already been added!"),
            None => self.keys = Some(EdgeKeys::new(key, tail_key, head_key))
        }
    }

    fn pos_changeable(&self) {
        if self.keys.is_some() {
            panic!("Direct change of edge position is available only when it is not added to graph");
        }
    }

    fn initialized(&self) {
        if self.keys.is_none() {
            panic!("Edge is not initialized");
        }
    }

    fn set_p1_uns(&mut self, x: f32, y: f32) {
        self.pos.x1 = x; self.pos.y1 = y;
    }

    fn set_p2_uns(&mut self, x: f32, y: f32) {
        self.pos.x2 = x; self.pos.y2 = y;
    }

    fn set_p1_t_uns(&mut self, t: (f32, f32)) {
        self.pos.x1 = t.0; self.pos.y1 = t.1;
    }

    fn set_p2_t_uns(&mut self, t: (f32, f32)) {
        self.pos.x2 = t.0; self.pos.y2 = t.1;
    }

    fn set_p1_v_uns(&mut self, v: &Vec2) {
        self.pos.x1 = v.x; self.pos.y1 = v.y
    }

    fn set_p2_v_uns(&mut self, v: &Vec2) {
        self.pos.x2 = v.x; self.pos.y2 = v.y
    }
}