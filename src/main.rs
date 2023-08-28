use std::fs;
use egui_macroquad::egui;
use egui_macroquad::egui::{FontId};
use macroquad::camera::{
    Camera2D,
    set_camera
};
use macroquad::math::{Vec2, vec2};
use macroquad::rand::ChooseRandom;
use macroquad::time::get_frame_time;
use rand::prelude::*;
use macroquad_project::*;
use serde_json;




fn window_conf() -> Conf {
    Conf {
        window_title: TITLE.to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Conf::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    prevent_quit();
    let mut graph = match fs::read_to_string(GRAPH_PATH) {
        Ok(str) => serde_json::from_str(&str).expect("graph loading error"),
        Err(e) => {
            println!("{}", e);
            Graph::new()
        }
    };

    let mut held_sn_k_v: Option<(NodeKey, Vec2)> = None;
    let mut shift_held_sn_k: Option<NodeKey> = None;
    let mut start_n_k = None;
    let mut finish_n_k = None;
    let mut draw_lengths = DRAW_LENGHTS;

    let mut cam: Camera2D = Camera2D::default();
    cam.zoom = 2.0/vec2(WIDTH, -HEIGHT);


    let help = "WASD        - movement
'+', '-'    - zoom
LMB         - create node, hold and drag
              to create edge with node
              or join existing nodes
Shift + LMB - move node
RMB         - delete node or edge
C           - hold to delete any selected element
Shift + C   - delete graph
1           - set start for path finding
2           - set finish for path finding
G + N       - generate nodes
G + E       - generate edges

when start and finish are set shortest path will be calculated if it exists";




    while !is_key_pressed(KeyCode::Escape) && !is_quit_requested() {
        let mut mouse_over_ui = false;

        egui_macroquad::ui(|ctx| {
            mouse_over_ui = ctx.is_pointer_over_area();

            egui::Window::new("how to use")
                .resizable(false)
                .show(ctx, |ui| {

                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(help)
                                .font(FontId::monospace(12.0))
                        ).wrap(true)
                    );
                    ui.checkbox(&mut draw_lengths, "draw edge lengths");
                });
        });


        let m_v = cam.screen_to_world(Vec2::from(mouse_position()));
        let mut selected = graph.selected_k_v(&m_v);
        if !mouse_over_ui {
            //Key1
            if is_key_pressed(KeyCode::Key1) {
                if let SL::Node(sn_k) = selected {
                    match (start_n_k == Some(sn_k), finish_n_k == Some(sn_k)) {
                        (true, true) => unreachable!(),
                        (false, false) => start_n_k = Some(sn_k),
                        (true, false) => start_n_k = None,
                        (false, true) => {
                            start_n_k = Some(sn_k);
                            finish_n_k = None;
                        }
                    }
                }
            }
            //Key2
            else if is_key_pressed(KeyCode::Key2) {
                if let SL::Node(sn_k) = selected {
                    match (start_n_k == Some(sn_k), finish_n_k == Some(sn_k)) {
                        (true, true) => unreachable!(),
                        (false, false) => finish_n_k = Some(sn_k),
                        (false, true) => finish_n_k = None,
                        (true, false) => {
                            finish_n_k = Some(sn_k);
                            start_n_k = None;
                        }
                    }
                }
            }

            //Shift + LMB
            else if is_key_down(KeyCode::LeftShift) && is_mouse_button_pressed(MouseButton::Left) {
                if let SL::Node(sn_k) = selected {
                    graph.set_node_pos(sn_k, &m_v);
                    shift_held_sn_k = Some(sn_k);
                }
            }

            //LMB pressed
            else if is_mouse_button_pressed(MouseButton::Left) {
                match selected {
                    SL::Node(sn_k) => {
                        let sn_v = graph.nodes[sn_k].get_p_v();
                        held_sn_k_v = Some((sn_k, sn_v));
                    }
                    SL::Edge(_) => {}
                    SL::None => {
                        let n_k = graph.add_node(Node::from(m_v));
                        held_sn_k_v = Some((n_k, m_v));
                    }
                }
            }


            //LMB released
            else if is_mouse_button_released(MouseButton::Left) {
                match selected {
                    SL::Node(sn_k) => {
                        let sn_v = graph.nodes[sn_k].get_p_v();

                        if let Some((sn0_k, sn0_v)) = held_sn_k_v {
                            if sn0_k != sn_k {
                                graph.add_edge(Edge::from(sn0_v, sn_v), sn0_k, sn_k);
                                if !ORIENTED { graph.add_edge(Edge::from(sn_v, sn0_v), sn_k, sn0_k); }
                            }
                        }
                    }
                    SL::Edge(_) | SL::None => {
                        if let Some((sn0_k, sn0_v)) = held_sn_k_v {
                            let dn_k = graph.add_node(Node::from(m_v));

                            graph.add_edge(Edge::from(sn0_v, m_v), sn0_k, dn_k);
                            if !ORIENTED { graph.add_edge(Edge::from(m_v, sn0_v), dn_k, sn0_k); }
                        }
                    }
                }
                held_sn_k_v = None;
            }

            //RMB or hold C
            else if is_mouse_button_pressed(MouseButton::Right) || is_key_down(KeyCode::C) {
                match selected {
                    SL::Node(sn_k) => {
                        graph.remove_node(sn_k);
                        selected = SL::None;
                    }
                    SL::Edge(se_k) => {
                        graph.remove_edge(se_k);
                        selected = SL::None;
                    }
                    SL::None => {}
                }
            }

            // Shift + C
            if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::C) {
                graph = Graph::new();
                held_sn_k_v = None;
                shift_held_sn_k = None;
                start_n_k = None;
                finish_n_k = None;
                selected = SL::None;
            }


            //G + N
            if is_key_down(KeyCode::G) && is_key_down(KeyCode::N) && !graph.nodes.is_empty() {
                let n_k = graph.nodes.keys().choose(&mut thread_rng()).unwrap();
                let n_v = graph.nodes[n_k].get_p_v();


                let rv = n_v + Vec2::new((random::<f32>() - 0.5) * 2.0 * MAX_SPAWN_NEIGHBOUR_DIST,
                                         (random::<f32>() - 0.5) * 2.0 * MAX_SPAWN_NEIGHBOUR_DIST);
                if graph.find_nodes(rv, 0.0, MIN_NEIGHBOUR_DIST).is_empty() &&
                    !graph.find_nodes(rv, 0.0, MAX_SPAWN_NEIGHBOUR_DIST).is_empty() {
                    graph.add_node(Node::from(rv));
                }
            }


            //G + E
            if is_key_down(KeyCode::G) && is_key_down(KeyCode::E) && !graph.nodes.is_empty() {
                let n_k = graph.nodes.keys().choose(&mut thread_rng()).unwrap();
                let n_v = graph.nodes[n_k].get_p_v();

                if let Some(&rn_k) = graph.find_nodes(n_v, MINIMUM_EDGE_LENGTH, MAXIMUM_EDGE_LENGTH).choose() {
                    let rn_v = graph.nodes[rn_k].get_p_v();

                    graph.add_edge(Edge::from(n_v, rn_v), n_k, rn_k);
                }
            }


            if let Some(sn_k) = shift_held_sn_k {
                graph.set_node_pos(sn_k, &m_v);
            }
            if is_mouse_button_released(MouseButton::Left) || is_key_released(KeyCode::LeftShift) {
                if shift_held_sn_k.is_some() {
                    shift_held_sn_k = None;
                }
            }
        }


        {
            let cam_speed = 300.0 * get_frame_time();
            let zoom_speed = 1.2;
            if is_key_down(KeyCode::W) { cam.target += vec2(0.0, -cam_speed); }
            if is_key_down(KeyCode::S) { cam.target += vec2(0.0, cam_speed); }
            if is_key_down(KeyCode::A) { cam.target += vec2(-cam_speed, 0.0); }
            if is_key_down(KeyCode::D) { cam.target += vec2(cam_speed, 0.0); }
            if is_key_pressed(KeyCode::Equal) { cam.zoom *= zoom_speed; }
            if is_key_pressed(KeyCode::Minus) { cam.zoom /= zoom_speed; }
            set_camera(&cam);
        }



        // drawing
        clear_background(BACKGROUND_COLOR);
        set_camera(&cam);



        if let Some(n_k) = start_n_k {
            match graph.nodes.get(n_k) {
                Some(n) => n.draw(PF_NODE_RADIUS, PF_START_COLOR),
                None => start_n_k = None
            }
        }
        if let Some(n_k) = finish_n_k {
            match graph.nodes.get(n_k) {
                Some(n) => n.draw(PF_NODE_RADIUS, PF_FINISH_COLOR),
                None => finish_n_k = None
            }
        }
        if !mouse_over_ui {
            match selected {
                SL::Node(sn_k) => {
                    let sn = &graph.nodes[sn_k];
                    sn.draw(SELECTED_NODE_RADIUS, SELECTED_NODE_COLOR);
                    let sn_v = sn.get_p_v();
                    if let Some((_, sn0_v)) = held_sn_k_v {
                        Edge::from(sn0_v, sn_v).draw(EDGE_THICKNESS, DRAG_EDGE_COLOR);
                    }
                }

                SL::Edge(se_k) => {
                    let se = &graph.edges[se_k];
                    se.draw_selected(EDGE_THICKNESS, SELECTED_EDGE_THICKNESS, SELECTED_EDGE_COLOR);
                    if let Some((_, sn0_v)) = held_sn_k_v {
                        Edge::from(sn0_v, m_v).draw(EDGE_THICKNESS, DRAG_EDGE_COLOR);
                        Node::from(m_v).draw(NODE_RADIUS, DRAG_NODE_COLOR);
                    }
                }

                SL::None => {
                    if let Some((_, sn0_v)) = held_sn_k_v {
                        Edge::from(sn0_v, m_v).draw(EDGE_THICKNESS, DRAG_EDGE_COLOR);
                        Node::from(m_v).draw(NODE_RADIUS, DRAG_NODE_COLOR);
                    }
                }
            }
        }

        if let (Some(a), Some(b)) = (start_n_k, finish_n_k) {
            let (n_path, e_path, _d) = find_shortest_path(&graph, a, b);
            if n_path.len() > 1 {
                graph.draw_path_gradient(n_path, e_path);
            }
        }
        graph.draw_nodes();
        graph.draw_edges();
        if draw_lengths {graph.draw_lenghts();}





        egui_macroquad::draw();

        next_frame().await;
    }
    let json_str = serde_json::to_string_pretty(&graph).unwrap();
    fs::write(GRAPH_PATH, json_str.as_str()).expect(&*format!("error occurred while writing to {}", GRAPH_PATH));
    println!("end of program");
}