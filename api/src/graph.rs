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

pub type DistanceData = Vec<f64>;

pub type SubmissionData = HashMap<String, (f64, f64)>;

#[derive(Deserialize, Serialize)]
pub struct MetricsData {
    pub stress: Option<f64>,
}

pub fn compute_distance(graph: &GraphData) -> Vec<f64> {
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

pub fn compute_metrics(
    graph: &GraphData,
    distance: &Vec<f64>,
    drawing: &SubmissionData,
) -> MetricsData {
    MetricsData {
        stress: Some(compute_stress(graph, distance, drawing)),
    }
}

fn compute_stress(graph: &GraphData, distance: &Vec<f64>, drawing: &SubmissionData) -> f64 {
    let n = drawing.len();
    let mut s = 0.;
    for i in 1..n {
        for j in 0..i {
            let dx = drawing[&graph.nodes[i].id].0 - drawing[&graph.nodes[j].id].0;
            let dy = drawing[&graph.nodes[i].id].1 - drawing[&graph.nodes[j].id].1;
            let e = ((dx * dx + dy * dy).sqrt() - distance[i * n + j]) / distance[i * n + j];
            s += e * e;
        }
    }
    s
}
