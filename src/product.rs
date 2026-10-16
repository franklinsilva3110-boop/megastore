// src/product.rs
// Módulo responsável pela estrutura de Produto e Categoria

use std::collections::HashMap;

/// Representa uma categoria de produto
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Category {
    pub id: String,
    pub name: String,
}

/// Representa um produto no catálogo da MegaStore
#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub category: Category,
    pub price: f64,
}

/// Catálogo de produtos usando HashMap para acesso O(1) por ID
pub struct ProductCatalog {
    products: HashMap<String, Product>,
}

impl ProductCatalog {
    pub fn new() -> Self {
        ProductCatalog {
            products: HashMap::new(),
        }
    }

    /// Cadastra um novo produto
    pub fn add_product(&mut self, product: Product) {
        self.products.insert(product.id.clone(), product);
    }

    /// Consulta um produto pelo ID
    pub fn get_product(&self, id: &str) -> Option<&Product> {
        self.products.get(id)
    }

    /// Lista todos os produtos
    pub fn list_all(&self) -> Vec<&Product> {
        self.products.values().collect()
    }

    /// Retorna a quantidade de produtos cadastrados
    pub fn len(&self) -> usize {
        self.products.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_category() -> Category {
        Category {
            id: "C01".to_string(),
            name: "Eletrônicos".to_string(),
        }
    }

    #[test]
    fn test_add_and_get_product() {
        let mut catalog = ProductCatalog::new();
        let product = Product {
            id: "P001".to_string(),
            name: "Smartphone".to_string(),
            category: sample_category(),
            price: 1999.90,
        };

        catalog.add_product(product);
        assert_eq!(catalog.len(), 1);

        let found = catalog.get_product("P001").unwrap();
        assert_eq!(found.name, "Smartphone");
        assert_eq!(found.price, 1999.90);
    }

    #[test]
    fn test_list_all() {
        let mut catalog = ProductCatalog::new();
        catalog.add_product(Product {
            id: "P001".to_string(),
            name: "A".to_string(),
            category: sample_category(),
            price: 10.0,
        });
        catalog.add_product(Product {
            id: "P002".to_string(),
            name: "B".to_string(),
            category: sample_category(),
            price: 20.0,
        });

        let all = catalog.list_all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_get_nonexistent_product() {
        let catalog = ProductCatalog::new();
        assert!(catalog.get_product("INEXISTENTE").is_none());
    }
}
