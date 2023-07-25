use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GraphLink {
    pub source: String,
    pub target: String,
    pub weight: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
}

pub type DistanceData = Vec<Vec<f64>>;

#[derive(Serialize)]
pub struct Graph {
    pub contest_name: String,
    pub graph_name: String,
    pub content: sqlx::types::Json<GraphData>,
    pub distance: sqlx::types::Json<Vec<Vec<f64>>>,
}

#[derive(Serialize)]
pub struct GraphMeta {
    pub contest_name: String,
    pub graph_name: String,
}

#[derive(Serialize)]
pub struct GraphContent {
    pub content: sqlx::types::Json<GraphData>,
}

#[derive(Serialize)]
pub struct GraphDistance {
    pub distance: sqlx::types::Json<Vec<Vec<f64>>>,
}

#[derive(Deserialize, Serialize)]
pub struct UpsertGraph {
    pub contest_name: String,
    pub graph_name: String,
    pub content: sqlx::types::Json<GraphData>,
    pub distance: sqlx::types::Json<DistanceData>,
}

impl UpsertGraph {
    pub fn new(contest_name: String, graph_name: String, content: GraphData) -> UpsertGraph {
        let d = compute_distance(&content);
        UpsertGraph {
            contest_name,
            graph_name,
            content: sqlx::types::Json(content),
            distance: sqlx::types::Json(d),
        }
    }
}

fn compute_distance(graph: &GraphData) -> Vec<Vec<f64>> {
    let n = graph.nodes.len();
    let mut d = vec![vec![std::f64::INFINITY; n]; n];
    let mut node_index = HashMap::new();
    for (i, node) in graph.nodes.iter().enumerate() {
        node_index.insert(node.id.clone(), i);
        d[i][i] = 0.;
    }
    for link in graph.links.iter() {
        let u = node_index[&link.source];
        let v = node_index[&link.target];
        d[u][v] = link.weight;
        d[v][u] = link.weight;
    }
    d
}
