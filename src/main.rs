use macroquad_project::*;




#[macroquad::main("BasicShapes")]
async fn main() {
    request_new_screen_size(640.0, 480.0);
    let mut graph = Graph::new();

    let mut n1 = Node::new(100.0, 100.0);
    let mut n2 = Node::new(200.0, 150.0);
    let e = Edge::new(&mut n1, &mut n2);
    graph.add_node(n1);
    graph.add_node(n2);
    graph.add_edge(e);



    loop {
        clear_background(WHITE);
        let mp = Vec2::from(mouse_position());




        let opt: Option<(Node, Edge)> = if let Some(n) = graph.get_selected(&mp) {
            n.draw_selected();
            if is_mouse_button_pressed(MouseButton::Left) {
                let mut nd = Node::from(mp.clone());

                let ed = Edge::new(n, &mut nd);
                Some((nd, ed))
            }
            else {
                None
            }
        }
        else {
            if is_mouse_button_pressed(MouseButton::Left) {
                graph.add_node(Node::from(mp));
            }
            None
        };

        if let Some((mut nd, ed)) = opt {
            nd.set_pos(mp);
            graph.update_edge_pos(&nd);
            nd.draw();
            ed.draw();
            if is_mouse_button_released(MouseButton::Left) {
                graph.add_node(nd);
                graph.add_edge(ed);
            }
        }



        graph.draw();
        next_frame().await;
    }
}



