# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

Sistema de recomendação de produtos desenvolvido em **Rust** para a disciplina *Data Structure Strategy and Implementation*.

O projeto modela produtos e categorias como vértices de um grafo e utiliza arestas ponderadas (co-compra e pertencimento a categoria) para gerar recomendações relevantes por meio de **BFS**.

---

## Objetivo e Funcionamento

A MegaStore precisa recomendar produtos de forma mais inteligente do que simplesmente mostrar os mais vendidos ou da mesma categoria.

O **ConectaStore** resolve isso representando as relações entre produtos como um grafo:

- **Vértices**: Produtos e Categorias
- **Arestas**: Co-compra (produto ↔ produto) e pertencimento (produto ↔ categoria)
- **Algoritmo**: BFS (Busca em Largura) para encontrar produtos relacionados
- **Estruturas complementares**: HashMap (catálogo), HashSet (evitar duplicatas), VecDeque (fila do BFS)

O sistema permite:
- Cadastrar e consultar produtos
- Construir o grafo de relações
- Recomendar produtos a partir de um produto específico
- Recomendar produtos a partir do histórico de compras de um cliente
- Evitar recomendações duplicadas e produtos já comprados

---

## Tecnologias e Estruturas de Dados

| Tecnologia / Estrutura | Uso |
|------------------------|-----|
| **Rust** | Linguagem principal |
| **Lista de Adjacência** | Representação do grafo (eficiente para grafos esparsos) |
| **HashMap** | Catálogo de produtos (acesso O(1) por ID) |
| **HashSet** | Controle de vértices visitados e prevenção de duplicatas |
| **VecDeque** | Fila utilizada pelo algoritmo BFS |
| **BFS** | Algoritmo de percurso para gerar recomendações |

---

## Estrutura do Projeto

```
megastore/
├── src/
│   ├── main.rs          # Ponto de entrada e demonstração
│   ├── lib.rs           # Biblioteca (para testes de integração)
│   ├── product.rs       # Produto, Categoria e ProductCatalog
│   └── graph.rs         # Grafo e algoritmos de recomendação
├── tests/
│   └── integration_test.rs
├── Cargo.toml
└── README.md
```

---

## Como Compilar e Executar

### Pré-requisitos
- Rust instalado (https://rustup.rs)

### Compilar e executar (modo debug)
```bash
cargo run
```

### Compilar e executar (modo release — otimizado)
```bash
cargo run --release
```

---

## Como Executar os Testes

```bash
cargo test
```

Os testes cobrem:
- Cadastro e consulta de produtos
- Construção do grafo
- Recomendação sem duplicatas
- Recomendação para cliente (excluindo produtos já comprados)
- Fluxo completo de integração

---

## Exemplos de Uso

Ao executar `cargo run`, o sistema demonstra:

1. Cadastro e listagem de produtos
2. Consulta individual de um produto
3. Construção do grafo
4. Recomendação a partir de um produto
5. Recomendação a partir do histórico de um cliente
6. Medição de desempenho com diferentes volumes de dados

### Exemplo de saída (resumo)

```
=== ConectaStore — Sistema de Recomendação ===

Produtos cadastrados: 5
Lista completa de produtos:
  1. Smartphone X (ID: P001) - R$ 1999.90 [Eletrônicos]
  ...

Recomendações para quem viu o Smartphone X (P001):
  1. Fone de Ouvido Bluetooth (ID: P002)
  2. Capa de Celular (ID: P003)
  3. Carregador Rápido (ID: P004)
```

---

## Arquitetura da Solução

```
┌─────────────────┐     ┌──────────────────────┐
│  ProductCatalog │     │  RecommendationGraph │
│   (HashMap)     │     │  (Lista Adjacência)  │
└────────┬────────┘     └──────────┬───────────┘
         │                         │
         │                         │
         └──────────┬──────────────┘
                    │
            ┌───────▼────────┐
            │  BFS + HashSet │
            │  (Recomendação)│
            └────────────────┘
```

- **ProductCatalog**: armazena e consulta produtos rapidamente.
- **RecommendationGraph**: mantém as relações entre produtos e categorias.
- **BFS**: percorre o grafo a partir de um produto (ou conjunto de produtos) e retorna os mais relacionados, priorizando arestas de maior peso e evitando duplicatas.

---

## Resultados dos Testes de Desempenho

Medições realizadas com grafos sintéticos (cada produto conectado a 3 outros):

### Modo Release (`cargo run --release`)

| Produtos | Arestas | Tempo médio (ms) |
|----------|---------|------------------|
| 100      | 300     | ~0.005           |
| 500      | 1500    | ~0.004           |
| 1000     | 3000    | ~0.006           |
| 5000     | 15000   | ~0.004           |

### Modo Debug (`cargo run`)

| Produtos | Arestas | Tempo médio (ms) |
|----------|---------|------------------|
| 100      | 300     | ~0.015           |
| 500      | 1500    | ~0.029           |
| 1000     | 3000    | ~0.015           |
| 5000     | 15000   | ~0.016           |

**Conclusão**: A lista de adjacência + BFS apresenta bom desempenho mesmo com milhares de produtos, sendo adequada para catálogos grandes.

---

## Autor

Projeto desenvolvido para a disciplina **Data Structure Strategy and Implementation**.
