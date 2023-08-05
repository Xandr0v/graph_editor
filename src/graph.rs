use crate::{node::*, edge::*};
use slotmap::{new_key_type, SlotMap};
pub use slotmap::DefaultKey;
use macroquad::math::Vec2;
use ord_subset::OrdSubsetIterExt;

new_key_type! {
    pub struct NodeKey;
    pub struct EdgeKey;
}

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
        if let None = node.id {
            self.nodes.insert_with_key(|k| {
                    node.id = Some(NodeId::new(k));
                    node
            })
        } else {
            panic!("Node has already been added!");
        }
    }

    pub fn add_edge(&mut self, mut edge: Edge, tail_node_id: NodeKey, head_node_id: NodeKey) -> EdgeKey {
        if let None = edge.id {
            let id = self.edges.insert_with_key(|k| {
                edge.id = Some(EdgeId::new(k, tail_node_id, head_node_id));
                edge
            });

            if let Some(tail_node) = self.nodes.get_mut(tail_node_id) {
                tail_node.get_mut_id().tails.push(id);
            }
            if let Some(head_node) = self.nodes.get_mut(head_node_id) {
                head_node.get_mut_id().heads.push(id);
            }
            id
        }
        else {
            panic!("Edge has already been added!");
        }
    }



    pub fn draw(&self) {
        for (_k, n) in &self.nodes {
            n.draw();
        }
        for (_k, e) in &self.edges {
            e.draw();
        }
    }


    fn get_mut_node<'a>(nodes: &'a mut SlotMap<NodeKey, Node>, id: &'a NodeKey) -> &'a mut Node {
        nodes.get_mut(*id).expect("Node is not found")
    }

    fn get_mut_edge<'a>(edges: &'a mut SlotMap<EdgeKey, Edge>, id: &'a EdgeKey) -> &'a mut Edge {
        edges.get_mut(*id).expect("Edge is not found")
    }



    pub fn set_node_pos(&mut self, id: &NodeKey, p: &Vec2) {

        let node = Self::get_mut_node(&mut self.nodes, id);
        node.p = *p;


        for he_id in &node.get_id().heads {
            Self::get_mut_edge(&mut self.edges, he_id).p2 = *p;
        }
        for te_id in &node.get_id().tails {
            Self::get_mut_edge(&mut self.edges, te_id).p1 = *p;
        }
    }




    fn square_dist(a: &Vec2, b: &Vec2) -> f32 {
        (a.x - b.x).powi(2) + (a.y - b.y).powi(2)
    }


    pub fn selected_node_id(&mut self, mp: &Vec2) -> Option<NodeKey> {
        self.nodes
            .iter_mut()
            .filter(|(_id, n)| Self::square_dist(&n.p, mp) < n.rad *n.rad)
            .ord_subset_min_by_key(|(_id, n)| Self::square_dist(&n.p, mp))
            .map(|(id, _n)| id)
    }



}