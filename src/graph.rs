// src/graph.rs
// Grafo de recomendação usando Lista de Adjacência

use std::collections::{HashMap, HashSet, VecDeque};

/// Tipo de vértice no grafo
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VertexType {
    Product(String),   // ID do produto
    Category(String),  // ID da categoria
}

/// Aresta ponderada
#[derive(Debug, Clone)]
pub struct Edge {
    pub target: VertexType,
    pub weight: f64,
}

/// Grafo não direcionado e ponderado com Lista de Adjacência
pub struct RecommendationGraph {
    // Lista de adjacência: vértice -> lista de arestas
    adjacency: HashMap<VertexType, Vec<Edge>>,
}

impl RecommendationGraph {
    pub fn new() -> Self {
        RecommendationGraph {
            adjacency: HashMap::new(),
        }
    }

    /// Adiciona um vértice ao grafo
    pub fn add_vertex(&mut self, vertex: VertexType) {
        self.adjacency.entry(vertex).or_insert_with(Vec::new);
    }

    /// Adiciona uma aresta não direcionada (conecta nos dois sentidos)
    pub fn add_edge(&mut self, from: VertexType, to: VertexType, weight: f64) {
        // Garante que os vértices existam
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());

        // Adiciona a aresta em ambos os sentidos (grafo não direcionado)
        self.adjacency.get_mut(&from).unwrap().push(Edge {
            target: to.clone(),
            weight,
        });
        self.adjacency.get_mut(&to).unwrap().push(Edge {
            target: from,
            weight,
        });
    }

    /// Retorna os vizinhos de um vértice
    pub fn neighbors(&self, vertex: &VertexType) -> Option<&Vec<Edge>> {
        self.adjacency.get(vertex)
    }

    /// Retorna a quantidade de vértices
    pub fn vertex_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Recomenda produtos relacionados a partir de um produto usando BFS.
    /// 
    /// - `start_product_id`: ID do produto de origem
    /// - `max_results`: quantidade máxima de recomendações
    /// - `exclude`: produtos que devem ser ignorados (ex: já comprados)
    /// 
    /// Retorna uma lista de IDs de produtos recomendados, sem duplicatas.
    pub fn recommend_by_product(
        &self,
        start_product_id: &str,
        max_results: usize,
        exclude: &HashSet<String>,
    ) -> Vec<String> {
        let start = VertexType::Product(start_product_id.to_string());

        // Se o produto não existe no grafo, retorna vazio
        if !self.adjacency.contains_key(&start) {
            return Vec::new();
        }

        let mut recommendations = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Marca o produto inicial como visitado e enfileira
        visited.insert(start.clone());
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            // Já temos recomendações suficientes
            if recommendations.len() >= max_results {
                break;
            }

            if let Some(edges) = self.neighbors(&current) {
                // Ordena as arestas por peso (maior primeiro) para priorizar relações mais fortes
                let mut sorted_edges = edges.clone();
                sorted_edges.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));

                for edge in sorted_edges {
                    let neighbor = edge.target.clone();

                    // Pula se já foi visitado
                    if visited.contains(&neighbor) {
                        continue;
                    }

                    visited.insert(neighbor.clone());

                    // Só recomenda se for um Produto e não estiver na lista de exclusão
                    if let VertexType::Product(ref product_id) = neighbor {
                        if !exclude.contains(product_id) && product_id != start_product_id {
                            recommendations.push(product_id.clone());
                            if recommendations.len() >= max_results {
                                break;
                            }
                        }
                    }

                    // Continua a busca (BFS) mesmo que seja categoria
                    queue.push_back(neighbor);
                }
            }
        }

        recommendations
    }

    /// Recomenda produtos para um cliente com base no seu histórico de compras.
    /// 
    /// Percorre as recomendações de cada produto comprado e une os resultados,
    /// removendo duplicatas e produtos já comprados.
    pub fn recommend_for_client(
        &self,
        purchased_product_ids: &[String],
        max_results: usize,
    ) -> Vec<String> {
        let exclude: HashSet<String> = purchased_product_ids.iter().cloned().collect();
        let mut all_recommendations = Vec::new();
        let mut seen = HashSet::new();

        for product_id in purchased_product_ids {
            let recs = self.recommend_by_product(product_id, max_results, &exclude);
            for rec in recs {
                if seen.insert(rec.clone()) {
                    all_recommendations.push(rec);
                    if all_recommendations.len() >= max_results {
                        return all_recommendations;
                    }
                }
            }
        }

        all_recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge_and_neighbors() {
        let mut graph = RecommendationGraph::new();
        graph.add_edge(
            VertexType::Product("P001".into()),
            VertexType::Product("P002".into()),
            5.0,
        );

        assert_eq!(graph.vertex_count(), 2);

        let neighbors = graph.neighbors(&VertexType::Product("P001".into())).unwrap();
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0].weight, 5.0);
    }

    #[test]
    fn test_recommend_by_product_no_duplicates() {
        let mut graph = RecommendationGraph::new();
        graph.add_edge(VertexType::Product("P001".into()), VertexType::Product("P002".into()), 5.0);
        graph.add_edge(VertexType::Product("P001".into()), VertexType::Product("P003".into()), 3.0);
        graph.add_edge(VertexType::Product("P002".into()), VertexType::Product("P003".into()), 2.0);

        let recs = graph.recommend_by_product("P001", 5, &HashSet::new());
        assert!(recs.contains(&"P002".to_string()));
        assert!(recs.contains(&"P003".to_string()));
        // Não deve ter duplicatas
        assert_eq!(recs.len(), 2);
    }

    #[test]
    fn test_recommend_for_client_excludes_purchased() {
        let mut graph = RecommendationGraph::new();
        graph.add_edge(VertexType::Product("P001".into()), VertexType::Product("P002".into()), 5.0);
        graph.add_edge(VertexType::Product("P001".into()), VertexType::Product("P003".into()), 4.0);
        graph.add_edge(VertexType::Product("P002".into()), VertexType::Product("P004".into()), 3.0);

        let purchases = vec!["P001".to_string(), "P002".to_string()];
        let recs = graph.recommend_for_client(&purchases, 5);

        // Não deve recomendar o que o cliente já comprou
        assert!(!recs.contains(&"P001".to_string()));
        assert!(!recs.contains(&"P002".to_string()));
        // Deve recomendar outros
        assert!(recs.contains(&"P003".to_string()) || recs.contains(&"P004".to_string()));
    }

    #[test]
    fn test_recommend_nonexistent_product() {
        let graph = RecommendationGraph::new();
        let recs = graph.recommend_by_product("INEXISTENTE", 3, &HashSet::new());
        assert!(recs.is_empty());
    }
}
