use crate::{node::Node, edge::Edge};
use indexmap::IndexMap;
use enum_as_inner::EnumAsInner;

use macroquad::math::Vec2;
use ord_subset::OrdSubsetIterExt;






pub struct Graph {
    id: u32,
    map: IndexMap<u32, GraphObject>
}

impl Graph {
    pub fn new() -> Self {
        Self {
            id: 0,
            map: IndexMap::new()
        }
    }

    pub fn set_node_id(&mut self, node: &mut Node) {
        if node.id == u32::MAX {
            node.id = self.id;
            self.id += 1;
        }
    }




    pub fn add_node(&mut self, mut node: Node) {
        self.set_node_id(&mut node);
        self.map.insert(node.id, GraphObject::Node(Box::new(node)));

    }

    pub fn add_edge(&mut self, edge: Edge) {
        if edge.id =
        self.map.insert(edge.id, GraphObject::Edge(Box::new(edge)));


    }


    pub fn get_node(&mut self, id: &u32) -> &mut Node {
        &mut **self.map.get_mut(id).unwrap().as_node_mut().unwrap()
    }

    pub fn get_edge(&mut self, id: &u32) -> &mut Edge {
        &mut **self.map.get_mut(id).unwrap().as_edge_mut().unwrap()
    }


    pub fn update_edge_pos(&mut self, n: &Node) {
        for id in &n.from_e_id {
            self.get_edge(id).p2 = n.p;
        }
        for id in &n.to_e_id {
            self.get_edge(id).p1 = n.p;
        }
    }



    pub fn draw(&self) {
        for (_i, obj) in &self.map {
            match obj {
                GraphObject::Node(n_box) => (*n_box).draw(),
                GraphObject::Edge(e_box) => (*e_box).draw()
            }
        }
    }


    fn square_dist(a: &Vec2, b: &Vec2) -> f32 {
        (a.x - b.x).powi(2) + (a.y - b.y).powi(2)
    }


    pub fn get_selected(&mut self, mp: &Vec2) -> Option<&mut Node> {
        self.map
            .iter_mut()
            .filter(|(_, obj)| obj.is_node())
            .map(|(_, obj)| &mut **(*obj).as_node_mut().unwrap())
            .filter(|n| Self::square_dist(&n.p, mp) < n.rad *n.rad)
            .ord_subset_min_by_key(|n| Self::square_dist(&n.p, mp))
    }

}


#[derive(EnumAsInner)]
enum GraphObject {
    Node(Box<Node>),
    Edge(Box<Edge>)
}