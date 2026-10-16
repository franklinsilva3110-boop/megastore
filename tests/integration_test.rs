// tests/integration_test.rs
// Testes de integração do ConectaStore

use megastore::product::{Category, Product, ProductCatalog};
use megastore::graph::{RecommendationGraph, VertexType};
use std::collections::HashSet;

#[test]
fn test_full_recommendation_flow() {
    // 1. Criar catálogo
    let mut catalog = ProductCatalog::new();
    let category = Category {
        id: "C01".to_string(),
        name: "Eletrônicos".to_string(),
    };

    catalog.add_product(Product {
        id: "P001".to_string(),
        name: "Smartphone".to_string(),
        category: category.clone(),
        price: 1999.90,
    });
    catalog.add_product(Product {
        id: "P002".to_string(),
        name: "Fone".to_string(),
        category: category.clone(),
        price: 299.90,
    });
    catalog.add_product(Product {
        id: "P003".to_string(),
        name: "Capa".to_string(),
        category: category,
        price: 49.90,
    });

    assert_eq!(catalog.len(), 3);

    // 2. Criar grafo
    let mut graph = RecommendationGraph::new();
    graph.add_edge(
        VertexType::Product("P001".into()),
        VertexType::Product("P002".into()),
        8.0,
    );
    graph.add_edge(
        VertexType::Product("P001".into()),
        VertexType::Product("P003".into()),
        5.0,
    );

    // 3. Recomendar
    let recs = graph.recommend_by_product("P001", 2, &HashSet::new());
    assert_eq!(recs.len(), 2);
    assert!(recs.contains(&"P002".to_string()));
    assert!(recs.contains(&"P003".to_string()));

    // 4. Recomendação para cliente
    let purchases = vec!["P001".to_string()];
    let client_recs = graph.recommend_for_client(&purchases, 5);
    assert!(!client_recs.contains(&"P001".to_string()));
    assert!(client_recs.len() >= 1);
}

#[test]
fn test_catalog_and_graph_integration() {
    let mut catalog = ProductCatalog::new();
    let cat = Category {
        id: "C02".to_string(),
        name: "Roupas".to_string(),
    };

    catalog.add_product(Product {
        id: "P010".to_string(),
        name: "Camiseta".to_string(),
        category: cat,
        price: 49.90,
    });

    assert!(catalog.get_product("P010").is_some());
    assert_eq!(catalog.list_all().len(), 1);
}
