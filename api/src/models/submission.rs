use crate::models::{DistanceData, Graph, GraphData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type SubmissionData = HashMap<String, (f64, f64)>;

#[derive(Deserialize, Serialize)]
pub struct MetricsData {
    stress: Option<f64>,
}

#[derive(Serialize)]
pub struct Submission {
    pub id: i32,
    pub contest_name: String,
    pub graph_name: String,
    pub user_id: String,
}

#[derive(Serialize)]
pub struct SubmissionContent {
    pub content: sqlx::types::Json<SubmissionData>,
}

#[derive(Serialize)]
pub struct SubmissionMetrics {
    pub metrics: sqlx::types::Json<MetricsData>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateSubmission {
    pub contest_name: String,
    pub graph_name: String,
    pub user_id: String,
    pub content: sqlx::types::Json<SubmissionData>,
    pub metrics: sqlx::types::Json<MetricsData>,
}

impl CreateSubmission {
    pub fn new(graph: Graph, user_id: String, content: SubmissionData) -> CreateSubmission {
        let metrics = compute_metrics(&graph.content, &graph.distance, &content);
        CreateSubmission {
            contest_name: graph.contest_name,
            graph_name: graph.graph_name,
            user_id,
            content: sqlx::types::Json(content),
            metrics: sqlx::types::Json(metrics),
        }
    }
}

fn compute_metrics(
    graph: &GraphData,
    distance: &DistanceData,
    drawing: &SubmissionData,
) -> MetricsData {
    MetricsData {
        stress: Some(compute_stress(graph, distance, drawing)),
    }
}

fn compute_stress(graph: &GraphData, distance: &DistanceData, drawing: &SubmissionData) -> f64 {
    let n = drawing.len();
    let mut s = 0.;
    for i in 1..n {
        for j in 0..i {
            let dx = drawing[&graph.nodes[i].id].0 - drawing[&graph.nodes[j].id].0;
            let dy = drawing[&graph.nodes[i].id].1 - drawing[&graph.nodes[j].id].1;
            let e = ((dx * dx + dy * dy).sqrt() - distance[i][j]) / distance[i][j];
            s += e * e;
        }
    }
    s
}
