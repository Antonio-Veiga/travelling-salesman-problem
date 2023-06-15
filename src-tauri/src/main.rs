// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// importa as estruturas de graph_defs.rs
use crate::graph_defs::{Graph, UndirectedPerformantGraph};
mod graph_defs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_resolving_tsp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
 *
 * Comando usado pelo frontend para enviar o grafo em JSON string para o backend
 */
#[tauri::command(rename_all = "snake_case")]
fn start_resolving_tsp(invoke_message: String) {
    // faz parse da string para um struct (Graph de graph_defs.rs)
    let parsing_result: Result<Graph, Box<dyn std::error::Error>> =
        parse_json_graph(invoke_message);

    // se não houver nenhum erro ao fazer o parsing inicia o processo de resolução do tsp (travelling salesman problem)
    if let Ok(graph) = parsing_result {
        solve_tsp(graph);
    }
}

// função de deserialização da JSON string para um struct (Graph de graph_defs.rs)
fn parse_json_graph(json: String) -> Result<Graph, Box<dyn std::error::Error>> {
    let parsed_graph: Graph = match serde_json::from_str(&json) {
        Ok(graph) => graph,
        Err(error) => return Err(Box::new(error)),
    };
    Ok(parsed_graph)
}

// função de resolução do tsp (travelling salesman problem)
// fn solve_tsp(graph: Graph) {
//     let up_graph: UndirectedPerformantGraph = create_undirected_performant_graph(graph);

//     let starting_vertex = match up_graph.get_key_vertex() {
//         Some(initial_vertex_value) => initial_vertex_value,
//         None => {
//             println!("Fatal: O nó de inicio não está definido.");
//             return;
//         }
//     };

//     let vertices = match up_graph.get_vertices() {
//         Some(vertices_values) => vertices_values,
//         None => {
//             println!("Fatal: Não existem nós no grafo.");
//             return;
//         }
//     };

//     let mut visited_vertices: Vec<u32> = vertices.into();
//     let mut optimal_path: Vec<u32> = vec![starting_vertex];
//     let mut total_weight: u32 = 0;

//     let mut curr_vertex = starting_vertex;

//     while !visited_vertices.is_empty() {
//         visited_vertices.retain(|&v| v != curr_vertex);

//         let neighbours = match up_graph.get_neighbors(curr_vertex) {
//             Some(neighbour_values) => neighbour_values,
//             None => {
//                 println!("Fatal: O grafo não é completo.");
//                 return;
//             }
//         };

//         let possible_path: Vec<(u32, u32)> = neighbours
//             .iter()
//             .filter(|&&(v, _)| visited_vertices.contains(&v))
//             .copied()
//             .collect();

//         let nearest_neighbour = match possible_path.iter().min_by_key(|&&(_, weight)| weight) {
//             Some(&v) => v,
//             None => {
//                 println!("Fatal: O grafo não é completo.");
//                 return;
//             }
//         };

//         total_weight += nearest_neighbour.1;
//         curr_vertex = nearest_neighbour.0;
//         optimal_path.push(curr_vertex);
//     }

//     let final_vertex_neighbours = match up_graph.get_neighbors(curr_vertex) {
//         Some(neighbour_values) => neighbour_values,
//         None => {
//             println!("Fatal: O grafo não é completo.");
//             return;
//         }
//     };

//     let starting_point_connection = match final_vertex_neighbours
//         .iter()
//         .find(|&&(v, _)| v == starting_vertex)
//     {
//         Some(&v) => v,
//         None => {
//             println!("Fatal: O grafo não é completo.");
//             return;
//         }
//     };

//     optimal_path.push(starting_point_connection.0);
//     total_weight += starting_point_connection.1;

//     let optimal_path_labels: Vec<String> = optimal_path
//         .iter()
//         .filter_map(|&idx| up_graph.get_label(idx))
//         .collect();

//     let optimal_path_string: String = optimal_path_labels.join("->");

//     println!("O algoritmo terminou...");
//     println!("O caminho mais curto: {}", optimal_path_string);
//     println!("O peso total: {}", total_weight);
// }

fn create_undirected_performant_graph(json_graph: Graph) -> UndirectedPerformantGraph {
    let mut up_graph = UndirectedPerformantGraph::new();
    let mut idx: u32 = 1;

    for node in json_graph.nodes {
        up_graph.add_vertex(idx, node.id);
        idx += 1;
    }

    for edge in json_graph.edges {
        up_graph.add_edge(edge.from, edge.to, edge.weight);
    }

    up_graph.set_key_vertex(json_graph.starting_node);

    return up_graph;
}

fn solve_tsp(graph: Graph) {
    let up_graph: UndirectedPerformantGraph = create_undirected_performant_graph(graph);

    let starting_vertex = match up_graph.get_key_vertex() {
        Some(initial_vertex_value) => initial_vertex_value,
        None => {
            println!("Fatal: O nó de inicio não está definido.");
            return;
        }
    };

    let vertices = match up_graph.get_vertices() {
        Some(vertices_values) => vertices_values,
        None => {
            println!("Fatal: Não existem nós no grafo.");
            return;
        }
    };

    let mut visited_vertices: Vec<u32> = vertices.into();
    visited_vertices.retain(|&v| v != starting_vertex);

    let mut optimal_path: Vec<u32> = vec![starting_vertex];
    let mut best_weight: u32 = std::u32::MAX;
    let mut best_path: Option<Vec<u32>> = None;

    fn branch_and_bound(
        curr_vertex: u32,
        curr_weight: u32,
        visited: &mut Vec<u32>,
        path: &mut Vec<u32>,
        up_graph: &UndirectedPerformantGraph,
        best_weight: &mut u32,
        best_path: &mut Option<Vec<u32>>,
        starting_vertex: u32,
    ) {
        if visited.is_empty() {
            let final_vertex_neighbours = match up_graph.get_neighbors(curr_vertex) {
                Some(neighbour_values) => neighbour_values,
                None => {
                    println!("Fatal: O grafo não é completo.");
                    return;
                }
            };

            let starting_point_connection = match final_vertex_neighbours
                .iter()
                .find(|&&(v, _)| v == starting_vertex)
            {
                Some(&v) => v,
                None => {
                    println!("Fatal: O grafo não é completo.");
                    return;
                }
            };

            let final_weight = curr_weight + starting_point_connection.1;
            if final_weight < *best_weight {
                *best_weight = final_weight;
                *best_path = Some(path.clone());
            }
            return;
        }

        let neighbours = match up_graph.get_neighbors(curr_vertex) {
            Some(neighbour_values) => neighbour_values,
            None => {
                println!("Fatal: O grafo não é completo.");
                return;
            }
        };

        for &(next_vertex, weight) in neighbours {
            if visited.contains(&next_vertex) && curr_weight + weight < *best_weight {
                visited.retain(|&v| v != next_vertex);
                path.push(next_vertex);

                branch_and_bound(
                    next_vertex,
                    curr_weight + weight,
                    visited,
                    path,
                    up_graph,
                    best_weight,
                    best_path,
                    starting_vertex,
                );

                visited.push(next_vertex);
                path.pop();
            }
        }
    }

    branch_and_bound(
        starting_vertex,
        0,
        &mut visited_vertices,
        &mut optimal_path,
        &up_graph,
        &mut best_weight,
        &mut best_path,
        starting_vertex,
    );

    match best_path {
        Some(mut path) => {
            path.push(starting_vertex);
            let optimal_path_labels: Vec<String> = path
                .iter()
                .filter_map(|&idx| up_graph.get_label(idx))
                .collect();
            let optimal_path_string: String = optimal_path_labels.join("->");

            println!("O algoritmo terminou...");
            println!("O caminho mais curto: {}", optimal_path_string);
            println!("O peso total: {}", best_weight);
        }
        None => {
            println!("Fatal: O grafo não é completo.");
        }
    }
}
