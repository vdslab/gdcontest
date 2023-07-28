use crate::{auth::validate_user, error::Result, models::*, repositories::*};
use axum::{extract::Path, Extension, Json};
use axum_auth::AuthBasic;
use std::sync::Arc;

pub async fn list(
    Path(contest_name): Path<String>,
    Extension(graphs): Extension<Arc<GraphRepository>>,
) -> Result<Json<Vec<Graph>>> {
    graphs
        .find_all(&contest_name)
        .await
        .map(|records| Json(records))
}

pub async fn get(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<GraphRepository>>,
) -> Result<Json<Graph>> {
    graphs
        .find(&contest_name, &graph_name)
        .await
        .map(|record| Json(record))
}

pub async fn get_content(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<GraphRepository>>,
) -> Result<Json<GraphData>> {
    graphs
        .find_content(&contest_name, &graph_name)
        .await
        .map(|record| Json(record))
}

pub async fn admin_list(
    Path(contest_name): Path<String>,
    Extension(graphs): Extension<Arc<AdminGraphRepository>>,
    AuthBasic((user, password)): AuthBasic,
) -> Result<Json<Vec<AdminGraph>>> {
    validate_user(user, password).await?;
    graphs
        .find_all(&contest_name)
        .await
        .map(|records| Json(records))
}

pub async fn admin_get(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<AdminGraphRepository>>,
    AuthBasic((user, password)): AuthBasic,
) -> Result<Json<AdminGraph>> {
    validate_user(user, password).await?;
    graphs
        .find(&contest_name, &graph_name)
        .await
        .map(|record| Json(record))
}

pub async fn admin_get_content(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<AdminGraphRepository>>,
    AuthBasic((user, password)): AuthBasic,
) -> Result<Json<GraphData>> {
    validate_user(user, password).await?;
    graphs
        .find_content(&contest_name, &graph_name)
        .await
        .map(|record| Json(record))
}

pub async fn admin_get_distance(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<AdminGraphRepository>>,
    AuthBasic((user, password)): AuthBasic,
) -> Result<Json<Vec<Vec<f64>>>> {
    validate_user(user, password).await?;
    let distance = graphs.find_distance(&contest_name, &graph_name).await?;
    let distance = distance_from_bytes(distance);
    let n = (distance.len() as f64).sqrt() as usize;
    let mut response = vec![vec![0.; n]; n];
    for i in 0..n {
        for j in 0..n {
            response[i][j] = distance[i * n + j];
        }
    }
    Ok(Json(response))
}

pub async fn admin_put(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<AdminGraphRepository>>,
    AuthBasic((user, password)): AuthBasic,
    Json(graph): Json<GraphData>,
) -> Result<Json<AdminGraph>> {
    validate_user(user, password).await?;
    let graph = AdminUpsertGraph::new(contest_name, graph_name, graph);
    graphs.save(&graph).await.map(|record| Json(record))
}
