use std::fs;
use egui_macroquad::egui;
use egui_macroquad::egui::{Align, Color32, Pos2, Visuals};
use egui_macroquad::egui::epaint::Shadow;

use macroquad::camera::{
    Camera2D,
    set_camera
};

use macroquad::math::{Vec2, vec2};
use macroquad::prelude::load_texture;
use macroquad::rand::ChooseRandom;
use macroquad::texture::{draw_texture, Texture2D};

use rand::prelude::*;
use macroquad_project::*;
use serde_json;
use slotmap::{Key, SecondaryMap};


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
    let mut graph;

    match fs::read_to_string(GRAPH_PATH) {
        Ok(str) => match serde_json::from_str(&str) {
            Ok(g) => graph = g,
            Err(e) => {
                println!("{}| graph resetted", e);
                graph = Graph::new();
            }
        }
        Err(e) => {
            println!("{}| graph resetted", e);
            graph = Graph::new();
        }
    };

    let mut held_sn_k_v: Option<(NodeKey, Vec2)> = None;
    let mut shift_held_sn_k: Option<NodeKey> = None;
    let mut start_n_k = None;
    let mut finish_n_k = None;
    let mut map: Option<Texture2D> = None;


    let mut undirected = UNDIRECTED;
    let mut draw_lengths = DRAW_LENGHTS;
    let mut no_neighbour_spawn_dist = NO_NEIGHBOUR_SPAWN_DIST;
    let mut max_neighbour_spawn_dist = MAX_NEIGHBOUR_SPAWN_DIST;
    let mut min_edge_length = MIN_EDGE_LENGTH;
    let mut max_edge_length = MAX_EDGE_LENGTH;
    let mut map_png_path = MAP_PNG_PATH.to_string();
    let mut load_map = None;

    let mut cam: Camera2D = Camera2D::default();
    cam.zoom = 2.0/vec2(WIDTH, -HEIGHT);

    let help = "Ctrl + LMB   - movement
LMB          - create node, hold and drag
               to create edge with node
               or join existing nodes
Shift + LMB  - move node
RMB          - hold to delete node or edge
mouse wheel  - zoom
S            - set start for path finding
F            - set finish for path finding
N            - generate nodes
E            - generate edges
T            - name node";


    let mut names: SecondaryMap<NodeKey, Box<String>> = SecondaryMap::new();



    while !is_key_pressed(KeyCode::Escape) && !is_quit_requested() {
        let mut mouse_over_ui = false;
        let m_v = cam.screen_to_world(Vec2::from(mouse_position()));
        let mut selected = graph.selected_k_v(&m_v);


        egui_macroquad::ui(|ctx| {
            mouse_over_ui = ctx.is_pointer_over_area();
            let mut visuals = Visuals::dark();



            visuals.window_shadow = Shadow::small_dark();
            visuals.widgets.noninteractive.fg_stroke.color = visuals.widgets.inactive.fg_stroke.color;
            visuals.extreme_bg_color = Color32::TRANSPARENT;

            ctx.set_visuals(visuals);

            egui::Window::new("Menu")
                .resizable(true)
                .default_width(200.0)
                .show(ctx, |ui| {
                    egui::CollapsingHeader::new("How to use")

                        .show(ui, |ui| {
                            ui.monospace(help);
                            ui.add_sized([300.0, 0.0], egui::Label::new(""));
                        });

                    egui::CollapsingHeader::new("Settings")
                        .show(ui, |ui| {
                            ui.checkbox(&mut undirected, "place undirected edges");
                            ui.checkbox(&mut draw_lengths, "draw edge lengths");

                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.add(egui::DragValue::new(&mut no_neighbour_spawn_dist).clamp_range(0.0..=500.0).speed(0.05));
                                ui.label("no neighbour spawn distance");
                            });
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.add(egui::DragValue::new(&mut max_neighbour_spawn_dist).clamp_range(0.0..=500.0).speed(0.05));
                                ui.label("max neighbour spawn distance");
                            });
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.add(egui::DragValue::new(&mut min_edge_length).clamp_range(no_neighbour_spawn_dist..=500.0).speed(0.05));
                                ui.label("min edge length");
                            });
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.add(egui::DragValue::new(&mut max_edge_length).clamp_range(min_edge_length..=500.0).speed(0.05));
                                ui.label("max edge length");
                            });
                            ui.label("");
                            if ui.button("default settings").clicked() {
                                undirected = UNDIRECTED;
                                draw_lengths = DRAW_LENGHTS;
                                no_neighbour_spawn_dist = NO_NEIGHBOUR_SPAWN_DIST;
                                max_neighbour_spawn_dist = MAX_NEIGHBOUR_SPAWN_DIST;
                                min_edge_length = MIN_EDGE_LENGTH;
                                max_edge_length = MAX_EDGE_LENGTH;
                            }
                        });

                    if ui.button("reset graph").clicked() {
                        graph = Graph::new();
                        held_sn_k_v = None;
                        shift_held_sn_k = None;
                        start_n_k = None;
                        finish_n_k = None;
                        selected = SL::None;
                        names.clear();
                    }
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.label("path: ");
                        ui.add(egui::TextEdit::singleline(&mut map_png_path).desired_width(f32::INFINITY));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui.button("load map").clicked() {
                                load_map = Some(map_png_path.clone());
                            }


                        });
                    });


                });


            for (n_k, box_name) in &mut names {
                let s_scr_p = cam.world_to_screen(graph.nodes[n_k].get_p_v() - Vec2::new(0.0, 15.0));
                egui::Area::new(n_k.data().as_ffi().to_string())
                    .pivot(egui::Align2::CENTER_BOTTOM)
                    .fixed_pos(Pos2::from(s_scr_p.to_array()))
                    .show(ctx, |ui| {
                        ui.add(egui::TextEdit::singleline(&mut **box_name)
                            .desired_width(65.0)
                            .text_color(Color32::BLACK)
                            .horizontal_align(Align::Center)
                            .vertical_align(Align::BOTTOM)
                        );
                    });

            }


        });


        if !mouse_over_ui {
            if is_key_pressed(KeyCode::T) {
                if let SL::Node(sn_k) = selected {
                    names.insert(sn_k, Box::new(String::from("Name")));
                }
            }




            //Key1
            if is_key_pressed(KeyCode::S) {
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
            else if is_key_pressed(KeyCode::F) {
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
            else if !is_key_down(KeyCode::LeftControl) {
                if is_key_down(KeyCode::LeftShift) && is_mouse_button_pressed(MouseButton::Left) {
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
            }
            //LMB released
            if is_mouse_button_released(MouseButton::Left) {
                match selected {
                    SL::Node(sn_k) => {
                        let sn_v = graph.nodes[sn_k].get_p_v();

                        if let Some((sn0_k, sn0_v)) = held_sn_k_v {
                            if sn0_k != sn_k {
                                graph.add_edge(Edge::from(sn0_v, sn_v), sn0_k, sn_k);
                                if undirected { graph.add_edge(Edge::from(sn_v, sn0_v), sn_k, sn0_k); }
                            }
                        }
                    }
                    SL::Edge(_) | SL::None => {
                        if let Some((sn0_k, sn0_v)) = held_sn_k_v {
                            let dn_k = graph.add_node(Node::from(m_v));

                            graph.add_edge(Edge::from(sn0_v, m_v), sn0_k, dn_k);
                            if undirected { graph.add_edge(Edge::from(m_v, sn0_v), dn_k, sn0_k); }
                        }
                    }
                }
                held_sn_k_v = None;
            }


            //RMB
            if is_mouse_button_down(MouseButton::Right) {
                match selected {
                    SL::Node(sn_k) => {
                        graph.remove_node(sn_k);
                        names.remove(sn_k);
                        selected = SL::None;
                    }
                    SL::Edge(se_k) => {
                        graph.remove_edge(se_k);
                        selected = SL::None;
                    }
                    SL::None => {}
                }
            }


            //N
            if is_key_down(KeyCode::N) && !graph.nodes.is_empty() {
                let n_k = graph.nodes.keys().choose(&mut thread_rng()).unwrap();
                let n_v = graph.nodes[n_k].get_p_v();


                let rv = n_v + Vec2::new((random::<f32>() - 0.5) * 2.0 * max_neighbour_spawn_dist,
                                         (random::<f32>() - 0.5) * 2.0 * max_neighbour_spawn_dist);
                if graph.find_nodes(rv, 0.0, no_neighbour_spawn_dist).is_empty() &&
                    !graph.find_nodes(rv, 0.0, max_neighbour_spawn_dist).is_empty() {
                    graph.add_node(Node::from(rv));
                }
            }


            //E
            if is_key_down(KeyCode::E) && !graph.nodes.is_empty() {
                let n_k = graph.nodes.keys().choose(&mut thread_rng()).unwrap();
                let n_v = graph.nodes[n_k].get_p_v();

                if let Some(&rn_k) = graph.find_nodes(n_v, min_edge_length, max_edge_length).choose() {
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
            let (_mwx, mwy) = mouse_wheel();
            if mwy > 0.0 {cam.zoom *= 1.02}
            else if mwy < 0.0 {cam.zoom /= 1.02}

            let mdp = mouse_delta_position();
            if is_key_down(KeyCode::LeftControl) && is_mouse_button_down(MouseButton::Left) {
                cam.target += mdp / cam.zoom * vec2(1.0, -1.0);
            }
            set_camera(&cam);
        }

        if let Some(path) = load_map {
            match load_texture(path.as_str()).await {
                Ok(texture) => map = Some(texture),
                Err(e) => {
                    println!("{}", e);
                    map = None;
                }
            }
            load_map = None;
        }

        // drawing
        clear_background(BACKGROUND_COLOR);
        set_camera(&cam);


        if let Some(ref texture) = map {
            draw_texture(*texture, 0.0, 0.0, WHITE);
        }

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