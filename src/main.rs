use macroquad_project::*;




#[macroquad::main("Graph")]
async fn main() {
    request_new_screen_size(640.0, 480.0);
    let mut graph = Graph::new();



    let mut drag = false;
    let mut dn_id = NodeKey::default();
    loop {
        clear_background(WHITE);
        let mp = Vec2::from(mouse_position());




        if let Some(sn_id) = graph.selected_node_id(&mp) {
            let sn = graph.nodes.get(sn_id).unwrap();
            sn.draw_selected();
            if is_mouse_button_pressed(MouseButton::Left) {
                let dn = Node::from(&mp);
                let de = Edge::new(sn.get_pos(), dn.get_pos());
                dn_id = graph.add_node(dn);
                graph.add_edge(de, sn_id, dn_id);
                drag = true;
            }
        }
        else if is_mouse_button_pressed(MouseButton::Left) {
                graph.add_node(Node::from(&mp));
        }

        if drag {
            graph.set_node_pos(&dn_id, &mp);

            if is_mouse_button_released(MouseButton::Left) {

                drag = false;
            }
        }



        graph.draw();
        next_frame().await;
    }
}



