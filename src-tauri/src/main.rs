// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// importa as estruturas de graph_defs.rs
use crate::graph_defs::{Graph, UndirectedPerformantGraph};
mod graph_defs;
use serde_json::json;
// use std::mem;
use std::sync::OnceLock;
use tauri::{Manager, Window};

static WINDOW: OnceLock<Window> = OnceLock::new();

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_resolving_tsp])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            _ = WINDOW.set(window);
            Ok(())
        })
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
    // cria um grafo não direcionado usando a função 'create_undirected_performant_graph'
    let up_graph: UndirectedPerformantGraph = create_undirected_performant_graph(graph);

    // obtém o vértice de partida usando a função 'get_key_vertex' do grafo
    let starting_vertex = match up_graph.get_key_vertex() {
        Some(initial_vertex_value) => initial_vertex_value,
        None => {
            print_to_logger(format!("Fatal: O nó de inicio não está definido."), false);
            println!("Fatal: O nó de inicio não está definido.");
            return;
        }
    };

    // obtém os vértices do grafo usando a função 'get_vertices'
    let vertices = match up_graph.get_vertices() {
        Some(vertices_values) => vertices_values,
        None => {
            print_to_logger(format!("Fatal: Não existem nós no grafo."), false);
            println!("Fatal: Não existem nós no grafo.");
            return;
        }
    };

    // cria um vetor 'visited_vertices' com todos os vértices, exceto o vértice de partida
    let mut visited_vertices: Vec<u32> = vertices.into();
    visited_vertices.retain(|&v| v != starting_vertex);

    // cria um vetor 'optimal_path' contendo apenas o vértice de partida
    // define o peso máximo com o maior valor possível
    // inicia 'best_path' como nulo
    let mut optimal_path: Vec<u32> = vec![starting_vertex];
    let mut best_weight: u32 = std::u32::MAX;
    let mut best_path: Option<Vec<u32>> = None;

    let mut iterations = 0;
    let mut instruction_count = 0;

    // função recursiva para encontrar o melhor caminho possível com busca em profundidade e 'poda'
    fn branch_and_bound(
        curr_vertex: u32,
        curr_weight: u32,
        visited: &mut Vec<u32>,
        path: &mut Vec<u32>,
        up_graph: &UndirectedPerformantGraph,
        best_weight: &mut u32,
        best_path: &mut Option<Vec<u32>>,
        starting_vertex: u32,
        iterations: &mut u32,
        instruction_count: &mut u32,
    ) {
        *iterations += 1;
        *instruction_count += 1;

        // em caso de todos os vértices terem sido visitados verfica se o vértice final tem ligação com o vértice inicial
        if visited.is_empty() {
            // obtém os vizinhos do vértice final
            let final_vertex_neighbours = match up_graph.get_neighbors(curr_vertex) {
                Some(neighbour_values) => neighbour_values,
                None => {
                    *instruction_count += 1;
                    // print_to_logger(format!("Sem vizinhos caminho incompatível."), false);
                    // println!("Sem vizinhos caminho incompatível.");
                    return;
                }
            };
            *instruction_count += 1;

            // verifica se o vértice inicial está entre os vizinhos do vértice final
            let starting_point_connection = match final_vertex_neighbours
                .iter()
                .find(|&&(v, _)| v == starting_vertex)
            {
                Some(&v) => v,
                None => {
                    *instruction_count += 1;
                    // print_to_logger(
                    //     format!("Não existe ligação ao nó inicial, caminho incompatível."),
                    //     false,
                    // );
                    println!("Não existe ligação ao nó inicial, caminho incompatível.");
                    return;
                }
            };

            *instruction_count += 1;

            // calcula o peso final do caminho
            let final_weight = curr_weight + starting_point_connection.1;
            // verifica se o peso final é menor que o melhor peso atual
            if final_weight < *best_weight {
                // atualiza o melhor peso
                *best_weight = final_weight;
                // armazena o caminho como o melhor caminho
                *best_path = Some(path.clone());
            }
            return;
        }

        // obtém os vizinhos do vértice atual
        let neighbours = match up_graph.get_neighbors(curr_vertex) {
            Some(neighbour_values) => neighbour_values,
            None => {
                *instruction_count += 1;
                // print_to_logger(format!("Sem vizinhos caminho incompatível."), false);
                println!("Sem vizinhos caminho incompatível.");
                return;
            }
        };

        *instruction_count += 1;

        // explora os vizinhos do vértice atual
        for &(next_vertex, weight) in neighbours {
            *instruction_count += 1;
            // verifica se o próximo vértice ainda não foi visitado e se o peso atual mais o peso da conexão é menor que o melhor peso atual
            if visited.contains(&next_vertex) && curr_weight + weight < *best_weight {
                // marca o próximo vértice como visitado
                visited.retain(|&v| v != next_vertex);
                // adiciona o próximo vértice ao caminho
                path.push(next_vertex);

                // print_to_logger(format!("Caminho atual: {:?}", path), false);
                // print_to_logger(format!("Peso atual: {:?}", curr_weight), false);

                // chamada recursiva para explorar o próximo vértice
                branch_and_bound(
                    next_vertex,
                    curr_weight + weight,
                    visited,
                    path,
                    up_graph,
                    best_weight,
                    best_path,
                    starting_vertex,
                    iterations,
                    instruction_count,
                );

                // desmarca o próximo vértice como visitado e remove-o do caminho para explorar outros vizinhos
                visited.push(next_vertex);
                path.pop();
            }
        }
    }

    // Log para o tempo de início
    let start_time = std::time::Instant::now();
    print_to_logger(format!("Algoritmo iniciado..."), false);

    // chama a função 'branch_and_bound' para iniciar a busca em profundidade com poda
    branch_and_bound(
        starting_vertex,
        0,
        &mut visited_vertices,
        &mut optimal_path,
        &up_graph,
        &mut best_weight,
        &mut best_path,
        starting_vertex,
        &mut iterations,
        &mut instruction_count,
    );

    // verifica o resultado da busca e imprime o caminho mais curto e o peso total
    match best_path {
        Some(mut path) => {
            path.push(starting_vertex);
            let optimal_path_labels: Vec<String> = path
                .iter()
                .filter_map(|&idx| up_graph.get_label(idx))
                .collect();
            let optimal_path_string: String = optimal_path_labels.join("->");

            let end_time = start_time.elapsed();
            print_to_logger(format!("Algoritmo terminado em {:?}", end_time), false);
            print_to_logger(
                format!("Caminho mais curto: {:?}", optimal_path_string),
                false,
            );
            print_to_logger(format!("Peso total: {:?}", best_weight), false);
            print_to_logger(
                format!("Número de instruções totais: {:?}", instruction_count),
                false,
            );
            print_to_logger(
                format!("Número de iterações recursivas: {:?}", iterations),
                true,
            );

            println!("O algoritmo terminou...");
            println!("O caminho mais curto: {}", optimal_path_string);
            println!("O peso total: {}", best_weight);
        }
        None => {
            print_to_logger(format!("Impossível encontrar o caminho mais curto"), true);
            println!("Impossível encontrar o caminho mais curto");
        }
    }
}

fn print_to_logger(message: String, unlock_graph: bool) {
    let event_payload = json!({
        "message": "Adding a value to the logger",
        "data": {
            "loggerMesage": message,
            "unlockGraph": unlock_graph
        }
    });

    if let Some(window) = WINDOW.get() {
        // print!("{:?}", window);
        if let Err(error) = window.emit("print-to-logger", event_payload) {
            println!("Error emitting 'print-to-logger' event: {}", error);
        }
    } else {
        println!("Window is not available");
    }
}
