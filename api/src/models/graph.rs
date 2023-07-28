use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphNode {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphLink {
    pub source: String,
    pub target: String,
    pub weight: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
}

pub type DistanceData = Vec<u8>;

#[derive(Serialize)]
pub struct Graph {
    pub contest_name: String,
    pub graph_name: String,
}

#[derive(Serialize)]
pub struct GraphContent {
    pub content: sqlx::types::Json<GraphData>,
}

#[derive(Serialize)]
pub struct GraphDistance {
    pub distance: DistanceData,
}

#[derive(Serialize)]
pub struct AdminGraph {
    pub contest_name: String,
    pub graph_name: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
pub struct AdminUpsertGraph {
    pub contest_name: String,
    pub graph_name: String,
    pub content: sqlx::types::Json<GraphData>,
    pub distance: DistanceData,
}

impl AdminUpsertGraph {
    pub fn new(contest_name: String, graph_name: String, content: GraphData) -> AdminUpsertGraph {
        let d = compute_distance(&content);
        AdminUpsertGraph {
            contest_name,
            graph_name,
            content: sqlx::types::Json(content),
            distance: distance_into_bytes(d),
        }
    }
}

fn compute_distance(graph: &GraphData) -> Vec<f64> {
    let n = graph.nodes.len();
    let mut distance = vec![std::f64::INFINITY; n * n];

    let mut node_index = HashMap::new();
    for (i, node) in graph.nodes.iter().enumerate() {
        node_index.insert(node.id.clone(), i);
    }

    let mut neighbors = vec![vec![]; n];
    for link in graph.links.iter() {
        let u = node_index[&link.source];
        let v = node_index[&link.target];
        neighbors[u].push((v, link.weight));
        neighbors[v].push((u, link.weight));
    }

    for s in 0..n {
        distance[s * n + s] = 0.;
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(OrderedFloat(distance[s * n + s])), s));
        while let Some((Reverse(OrderedFloat(d)), u)) = queue.pop() {
            if distance[s * n + u] < d {
                continue;
            }
            for &(v, c) in neighbors[u].iter() {
                if distance[s * n + u] + c < distance[s * n + v] {
                    distance[s * n + v] = distance[s * n + u] + c;
                    queue.push((Reverse(OrderedFloat(distance[s * n + v])), v));
                }
            }
        }
    }
    distance
}

pub fn distance_from_bytes(bytes: Vec<u8>) -> Vec<f64> {
    unsafe {
        std::slice::from_raw_parts(
            bytes.as_ptr() as *const f64,
            bytes.len() / std::mem::size_of::<f64>(),
        )
        .to_vec()
    }
}

pub fn distance_into_bytes(distance: Vec<f64>) -> Vec<u8> {
    unsafe {
        std::slice::from_raw_parts(
            distance.as_ptr() as *const u8,
            distance.len() * std::mem::size_of::<f64>(),
        )
        .to_vec()
    }
}
