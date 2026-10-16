// src/main.rs
// Ponto de entrada do ConectaStore

mod product;
mod graph;

use product::{Category, Product, ProductCatalog};
use graph::{RecommendationGraph, VertexType};

fn main() {
    println!("=== ConectaStore — Sistema de Recomendação ===\n");

    // =====================================================
    // 1. Catálogo de produtos (HashMap)
    // =====================================================
    let mut catalog = ProductCatalog::new();

    let cat_eletronicos = Category {
        id: "C01".to_string(),
        name: "Eletrônicos".to_string(),
    };

    let cat_roupas = Category {
        id: "C02".to_string(),
        name: "Roupas".to_string(),
    };

    let products = vec![
        Product {
            id: "P001".to_string(),
            name: "Smartphone X".to_string(),
            category: cat_eletronicos.clone(),
            price: 1999.90,
        },
        Product {
            id: "P002".to_string(),
            name: "Fone de Ouvido Bluetooth".to_string(),
            category: cat_eletronicos.clone(),
            price: 299.90,
        },
        Product {
            id: "P003".to_string(),
            name: "Capa de Celular".to_string(),
            category: cat_eletronicos.clone(),
            price: 49.90,
        },
        Product {
            id: "P004".to_string(),
            name: "Carregador Rápido".to_string(),
            category: cat_eletronicos.clone(),
            price: 89.90,
        },
        Product {
            id: "P005".to_string(),
            name: "Camiseta Básica".to_string(),
            category: cat_roupas.clone(),
            price: 49.90,
        },
    ];

    for p in &products {
        catalog.add_product(p.clone());
    }

    println!("Produtos cadastrados: {}", catalog.len());

    // Listar todos os produtos usando list_all
    println!("\nLista completa de produtos:");
    for (i, prod) in catalog.list_all().iter().enumerate() {
        println!("  {}. {} (ID: {}) - R$ {:.2} [{}]",
                 i + 1, prod.name, prod.id, prod.price, prod.category.name);
    }
    println!();

    // Exemplo de consulta individual
    if let Some(prod) = catalog.get_product("P001") {
        println!("Consulta individual (P001): {} - Categoria: {} - R$ {:.2}\n",
                 prod.name, prod.category.name, prod.price);
    }

    // =====================================================
    // 2. Construção do grafo
    // =====================================================
    let mut graph = RecommendationGraph::new();

    // Arestas de co-compra (Produto ↔ Produto)
    graph.add_edge(VertexType::Product("P001".into()), VertexType::Product("P002".into()), 8.0);
    graph.add_edge(VertexType::Product("P001".into()), VertexType::Product("P003".into()), 6.0);
    graph.add_edge(VertexType::Product("P001".into()), VertexType::Product("P004".into()), 4.0);
    graph.add_edge(VertexType::Product("P002".into()), VertexType::Product("P003".into()), 5.0);
    graph.add_edge(VertexType::Product("P002".into()), VertexType::Product("P004".into()), 3.0);

    // Arestas de categoria (Produto ↔ Categoria)
    for id in ["P001", "P002", "P003", "P004"] {
        graph.add_edge(
            VertexType::Product(id.into()),
            VertexType::Category("C01".into()),
            1.0,
        );
    }
    graph.add_edge(
        VertexType::Product("P005".into()),
        VertexType::Category("C02".into()),
        1.0,
    );

    println!("Vértices no grafo: {}\n", graph.vertex_count());

    // =====================================================
    // 3. Recomendação a partir de um produto
    // =====================================================
    println!("--- Recomendação a partir de um produto ---");
    let recs_product = graph.recommend_by_product("P001", 3, &std::collections::HashSet::new());

    println!("Para quem viu o Smartphone X (P001):");
    print_recommendations(&recs_product, &catalog);

    // =====================================================
    // 4. Recomendação a partir de um cliente (histórico de compras)
    // =====================================================
    println!("\n--- Recomendação a partir de um cliente ---");
    // Simulando que o cliente já comprou P001 e P002
    let client_purchases = vec!["P001".to_string(), "P002".to_string()];

    let recs_client = graph.recommend_for_client(&client_purchases, 3);

    println!("Cliente que comprou Smartphone X e Fone de Ouvido:");
    print_recommendations(&recs_client, &catalog);

    println!("\nSistema de recomendação funcionando corretamente!");

    // =====================================================
    // 5. Medição de desempenho
    // =====================================================
    println!("\n=== Medição de Desempenho ===");
    run_performance_tests();
}

/// Gera um grafo sintético e mede o tempo de recomendação
fn run_performance_tests() {
    use std::time::Instant;

    let volumes = [100, 500, 1000, 5000];

    println!("{:>10} | {:>12} | {:>15}", "Produtos", "Arestas", "Tempo (ms)");
    println!("{}", "-".repeat(42));

    for &n in &volumes {
        let mut graph = RecommendationGraph::new();

        // Cria n produtos e conecta cada um a alguns vizinhos (grafo esparso)
        for i in 0..n {
            let id = format!("P{:04}", i);
            // Conecta a 3 produtos "próximos" (simulando co-compra)
            for offset in 1..=3 {
                let neighbor = format!("P{:04}", (i + offset) % n);
                graph.add_edge(
                    VertexType::Product(id.clone()),
                    VertexType::Product(neighbor),
                    (offset as f64) * 2.0,
                );
            }
        }

        let start_id = "P0000".to_string();
        let start = Instant::now();

        // Executa a recomendação várias vezes para ter uma medição mais estável
        for _ in 0..10 {
            let _ = graph.recommend_by_product(&start_id, 10, &std::collections::HashSet::new());
        }

        let elapsed = start.elapsed();
        let avg_ms = elapsed.as_secs_f64() * 100.0; // média de 10 execuções em ms

        println!("{:>10} | {:>12} | {:>15.3}", n, n * 3, avg_ms);
    }

    println!("\nObservação: tempos medidos com a compilação atual.");
    println!("Use 'cargo run' para modo debug ou 'cargo run --release' para modo otimizado.");
}

/// Função auxiliar para imprimir recomendações de forma organizada
fn print_recommendations(rec_ids: &[String], catalog: &ProductCatalog) {
    if rec_ids.is_empty() {
        println!("  Nenhuma recomendação encontrada.");
        return;
    }

    for (i, id) in rec_ids.iter().enumerate() {
        if let Some(prod) = catalog.get_product(id) {
            println!("  {}. {} (ID: {}) - R$ {:.2} [{}]",
                     i + 1, prod.name, prod.id, prod.price, prod.category.name);
        }
    }
}
