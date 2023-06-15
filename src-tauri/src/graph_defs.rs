use serde::{Deserialize, Serialize};

/*
 *
 * Estas estruturas são apenas usadas para deserializar a JSON string de volta
 * numa estrutura de dados.
 *
 */

/*
 * Representa um nó na estrutura de deserialização.
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub label: String,
}

/*
 * Representa uma aresta na estrutura de deserialização.
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub weight: u32,
    pub label: String,
    pub id: String,
}

/*
 * Representa a estrutura de deserialização, o grafo em si.
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub starting_node: String,
}

/*
 *
 * Esta estrutura representa um grafo não ordenado através de uma lista de adjacências
 *
 */

use std::collections::HashMap;

// definição da estrutura de UndirectedPerformantGraph
#[derive(Debug, Serialize)]
pub struct UndirectedPerformantGraph {
    vertices: HashMap<VertexId, Vec<(VertexId, u32)>>,
    labels: HashMap<VertexId, String>,
    key_vertex: VertexId,
}

type VertexId = u32;

impl UndirectedPerformantGraph {
    // método construtor de UndirectedPerformantGraph
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            labels: HashMap::new(),
            key_vertex: 0,
        }
    }

    // método para adicionar um nó
    pub fn add_vertex(&mut self, vertex: VertexId, label: String) {
        // Verifica se o ID ou o rótulo já existem
        if self.labels.contains_key(&vertex) {
            println!(
                "Já existe um nó com o ID '{}', o nó não foi adicionado.",
                vertex
            );
        } else if self
            .labels
            .values()
            .any(|existing_label| *existing_label == label)
        {
            println!(
                "Já existe um nó com o rótulo '{}', o nó não foi adicionado.",
                label
            );
        } else {
            self.labels.insert(vertex, label);
        }
    }

    // método para adicionar uma ligação
    pub fn add_edge(&mut self, from_label: String, to_label: String, weight: u32) {
        // se o rótulo from existir
        if let Some(from) = self.find_vertex_id(from_label.clone()) {
            // se o rótulo to existir
            if let Some(to) = self.find_vertex_id(to_label.clone()) {
                if !self.vertices.contains_key(&from) {
                    self.vertices.insert(from, vec![]);
                }

                // Verifica se o nó "to" já existe nos vértices
                if !self.vertices.contains_key(&to) {
                    self.vertices.insert(to, vec![]);
                }

                // adiciona a aresta aos nós correspondentes
                self.vertices.get_mut(&from).unwrap().push((to, weight));
                self.vertices.get_mut(&to).unwrap().push((from, weight));

                return; // sai mais cedo se a ligação for adicionada com sucesso (Some)
            }
        }
        println!("Um ou ambos os nós não existem, a ligação não foi adicionada.");
    }

    pub fn set_key_vertex(&mut self, key_v_label: String) {
        if let Some(key) = self.find_vertex_id(key_v_label) {
            self.key_vertex = key;
            return; // sai mais cedo (Some)
        }
        println!("Impossível definir o nó chave o rótulo não existe");
    }

    // função que encontra o id do nó pelo seu rótulo
    fn find_vertex_id(&self, label: String) -> Option<VertexId> {
        for (vertex, vertex_label) in &self.labels {
            if *vertex_label == label {
                return Some(*vertex);
            }
        }
        None
    }

    // função que devolve o nó de início fim
    pub fn get_key_vertex(&self) -> Option<VertexId> {
        if self.key_vertex != 0 {
            Some(self.key_vertex)
        } else {
            None
        }
    }

    //função que devolve todos os nós do grafo
    pub fn get_vertices(&self) -> Option<Vec<VertexId>> {
        if self.labels.is_empty() {
            None
        } else {
            let keys: Vec<VertexId> = self.labels.keys().cloned().collect();
            Some(keys)
        }
    }

    // método que retorna os nós adjacentes a um determinado nó
    pub fn get_neighbors(&self, vertex: VertexId) -> Option<&Vec<(VertexId, u32)>> {
        self.vertices.get(&vertex)
    }

    // método para obter o rótulo de um nó
    pub fn get_label(&self, vertex: VertexId) -> Option<String> {
        self.labels.get(&vertex).cloned()
    }
}
