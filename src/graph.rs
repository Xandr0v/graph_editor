use std::f32::consts::PI;
use macroquad::color::{colors::*, rgb_to_hsl, hsl_to_rgb};
use crate::{Node, Edge, NodeGetSet, EdgeGetSet};
use macroquad::math::Vec2;
use macroquad::text::{draw_text_ex, TextParams};
use slotmap::{new_key_type, SlotMap};
use ord_subset::OrdSubsetIterExt;
use serde::{Serialize, Deserialize};
use crate::edge::EdgeGraph;
use crate::node_src::node::NodeGraph;
use crate::variables::*;



new_key_type! {
    pub struct NodeKey;
    pub struct EdgeKey;
}

pub enum SL {None, Node(NodeKey), Edge(EdgeKey)}


#[derive(Serialize, Deserialize)]
pub struct Graph {
    pub nodes: SlotMap<NodeKey, Node>,
    pub edges: SlotMap<EdgeKey, Edge>
}


impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            edges: SlotMap::with_key()
        }
    }

    pub fn add_node(&mut self, mut node: Node) -> NodeKey {
        self.nodes.insert_with_key(|k| {
            node.graph_init(k);
            node
        })
    }

    pub fn add_edge(&mut self, mut edge: Edge, tail_key: NodeKey, head_key: NodeKey) -> EdgeKey {

        for (e_k, e) in &self.edges {
            let keys = e.get_keys();
            if keys.from == tail_key && keys.to == head_key
                //|| keys.from == head_key && keys.to == tail_key
                {
                return e_k;
            }
        }

        let key = self.edges.insert_with_key(|k| {
            edge.graph_init(k, tail_key, head_key);
            edge
        });


        self.nodes[tail_key].get_mut_keys().tails.insert(key);
        self.nodes[head_key].get_mut_keys().heads.insert(key);
        key
    }

    pub fn remove_node(&mut self, key: NodeKey) {

        let mut n = self.nodes.remove(key).unwrap();
        let n_keys = n.get_mut_keys();

        for t_id in n_keys.tails.drain() {
            let e = self.edges.remove(t_id).unwrap();
            let e_keys = e.get_keys();

            self.nodes[e_keys.to].get_mut_keys().heads.remove(&e_keys.own);
        }

        for h_id in n_keys.heads.drain() {
            let e = self.edges.remove(h_id).unwrap();
            let e_keys = e.get_keys();

            self.nodes[e_keys.from].get_mut_keys().tails.remove(&e_keys.own);
        }
    }

    pub fn remove_edge(&mut self, key: EdgeKey) {
        let e = self.edges.remove(key).unwrap();
        let e_keys = e.get_keys();

        self.nodes[e_keys.from].get_mut_keys().tails.remove(&key);
        self.nodes[e_keys.to].get_mut_keys().heads.remove(&key);

    }


    pub fn set_node_pos(&mut self, key: NodeKey, v: &Vec2) {
        let n = &mut self.nodes[key];
        n.set_p_v_uns(v);

        for &he_id in &n.get_keys().heads {
            self.edges[he_id].set_p2_v_uns(v);
        }
        for &te_id in &n.get_keys().tails {
            self.edges[te_id].set_p1_v_uns(v);
        }

    }

    pub fn find_nearest_node(&self, key: NodeKey) -> Option<NodeKey>{
        let v = self.nodes[key].get_p_v();

        self.nodes
            .iter()
            .filter(|(k, _n)| *k != key)
            .ord_subset_min_by_key(|(_k, n)|
                Vec2::length_squared(n.get_p_v() - v)
            )
            .map(|(k, _n)| k)

    }

    pub fn find_near_nodes_by_key(&self, key: NodeKey, r: f32) -> Vec<NodeKey> {
        let v = self.nodes[key].get_p_v();

        self.nodes
            .iter()
            .filter(|(k, n)|
                *k != key &&
                    Vec2::length_squared(n.get_p_v() - v) < r*r
            )
            .map(|(k, _n)| k)
            .collect::<Vec<NodeKey>>()

    }





    pub fn find_nodes(&self, v: Vec2, mn_r: f32, mx_r: f32) -> Vec<NodeKey> {

        self.nodes
            .iter()
            .map(|(k, n)|
                     (k, Vec2::length_squared(n.get_p_v() - v))
            )
            .filter(|(_k, d)|
                *d != 0.0 && mn_r*mn_r < *d && *d < mx_r*mx_r
            )
            .map(|(k, _d)| k)
            .collect::<Vec<NodeKey>>()

    }






    pub fn selected_n_k(&self, mv: &Vec2) -> Option<NodeKey> {
        let mv = *mv;
        self.nodes
            .iter()
            .filter(|(_k, n)|
                Vec2::length_squared(n.get_p_v()-mv) < NODE_RADIUS.powi(2) * 4.0
            )
            .ord_subset_min_by_key(|(_k, n)|
                Vec2::length_squared(n.get_p_v()-mv)
            )
            .map(|(k, _n)| k)
    }


    pub fn selected_e_k(&self, mv: &Vec2) -> Option<EdgeKey> {
        let v0 = *mv;

        self.edges
            .iter()
            .filter(|(_k, e)| {
                let v1 = e.get_p1_v();
                let v2 = e.get_p2_v();
                let v12 = v2 - v1;
                let v10 = v0 - v1;
                let proj_unnorm = v10.dot(v12);
                let perp_proj = v10.perp_dot(v12.normalize());

                0.0 <= proj_unnorm && proj_unnorm <= v12.length_squared() && perp_proj.abs() <= EDGE_THICKNESS
            })
            .ord_subset_min_by_key(|(_k, e)| {
                let v1 = e.get_p1_v();
                let v2 = e.get_p2_v();
                let v12 = v2 - v1;
                let v10 = v0 - v1;

                let perp_proj_unnorm = v10.perp_dot(v12);
                perp_proj_unnorm
            })
            .map(|(k, _e)| k)
    }




    pub fn selected_k_v(&self, mv: &Vec2) -> SL {
        let mv = *mv;

        match (self.selected_n_k(&mv), self.selected_e_k(&mv)) {
            (Some(sn_k), Some(se_k)) => {
                let sn = &self.nodes[sn_k];
                let se = &self.edges[se_k];
                let nv = sn.get_p_v();
                let ev1 = se.get_p1_v();
                let ev2 = se.get_p2_v();
                let nd = nv.distance(mv);
                let ed = (mv - ev1).perp_dot((ev2 - ev1).normalize());

                if nd < ed || nd < SELECTED_NODE_RADIUS { SL::Node(sn_k) }
                else { SL::Edge(se_k) }
            }
            (Some(sn_k), None) => SL::Node(sn_k),
            (None, Some(se_k)) => SL::Edge(se_k),
            (None, None) => SL::None
        }
    }




    pub fn draw_nodes(&self) {
        for (_k, n) in &self.nodes {
            n.draw(NODE_RADIUS, NODE_COLOR);
        }
    }


    pub fn draw_edges(&self) {
        for (_k, e) in &self.edges {
            e.draw(EDGE_THICKNESS, EDGE_COLOR);
        }
    }

    pub fn draw_lenghts(&self) {
        for (_k, e) in &self.edges {
            let font_size = FONT_SIZE;
            let str = format!("{:.0}", e.get_length());
            let str_len = str.len() as f32;
            let v1 = e.get_p1_v();
            let v2 = e.get_p2_v();
            let vn = (v2 - v1).normalize();

            let mut rotation = f32::atan2(vn.y, vn.x);
            let t_v;
            match -PI/2.0 < rotation && rotation < PI/2.0 {
                true => {
                    t_v = (v1 + v2)/2.0
                        - vn * font_size * str_len / 4.0
                        - vn.perp() * 5.0;

                }
                false => {
                    t_v = (v1 + v2)/2.0
                        + vn * font_size * str_len / 4.0
                        + vn.perp() * 5.0;
                    rotation += PI;
                }
            }

            draw_text_ex(str.as_str(), t_v.x, t_v.y, TextParams {
                font_size: font_size as u16,
                color: BLACK,
                rotation,
                
                ..TextParams::default()
            });
        }
    }


    pub fn draw_path_gradient(&self, n_path: Vec<NodeKey>, e_path: Vec<EdgeKey>) {
        let s_hue = rgb_to_hsl(PF_START_COLOR).0;
        let f_hue = rgb_to_hsl(PF_FINISH_COLOR).0;
        let range = f_hue - s_hue;

        let mut e_hue = s_hue;
        let mut n_hue = s_hue;

        let e_hue_step = range / (e_path.len()-1) as f32;
        let n_hue_step = range / (n_path.len()-1) as f32;

        for e_k in e_path {
            if let Some(e) = self.edges.get(e_k) {
                e.draw_selected(EDGE_THICKNESS, PF_EDGE_THICKNESS, hsl_to_rgb(e_hue, 1.0, 0.5));
            }
            e_hue += e_hue_step;

        }

        for n_k in n_path {
            if let Some(n) = self.nodes.get(n_k) {
                n.draw(PF_NODE_RADIUS, hsl_to_rgb(n_hue, 1.0, 0.5));
            }
            n_hue += n_hue_step;
        }
    }



}




