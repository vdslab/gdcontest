use crate::{auth::Validator, error::Result, graph::*, repositories::*};
use axum::{extract::Path, Extension, Json};
use axum_auth::AuthBasic;
use std::sync::Arc;

pub async fn list<R>(
    Path(contest_name): Path<String>,
    Extension(graphs): Extension<Arc<R>>,
) -> Result<Json<Vec<Graph>>>
where
    R: GraphRepository,
{
    graphs
        .find_all(&contest_name)
        .await
        .map(|records| Json(records))
}

pub async fn get<R>(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<R>>,
) -> Result<Json<Graph>>
where
    R: GraphRepository,
{
    graphs
        .find(&contest_name, &graph_name)
        .await
        .map(|record| Json(record))
}

pub async fn get_content<R>(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<R>>,
) -> Result<Json<GraphData>>
where
    R: GraphRepository,
{
    graphs
        .find_content(&contest_name, &graph_name)
        .await
        .map(|record| Json(record))
}

pub async fn admin_list<R, V>(
    Path(contest_name): Path<String>,
    Extension(graphs): Extension<Arc<R>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
) -> Result<Json<Vec<Graph>>>
where
    R: GraphRepository,
    V: Validator,
{
    validator.validate_user(auth).await?;
    graphs
        .find_all(&contest_name)
        .await
        .map(|records| Json(records))
}

pub async fn admin_get<R, V>(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<R>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
) -> Result<Json<Graph>>
where
    R: GraphRepository,
    V: Validator,
{
    validator.validate_user(auth).await?;
    graphs
        .find(&contest_name, &graph_name)
        .await
        .map(|record| Json(record))
}

pub async fn admin_get_content<R, V>(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<R>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
) -> Result<Json<GraphData>>
where
    R: GraphRepository,
    V: Validator,
{
    validator.validate_user(auth).await?;
    graphs
        .find_content(&contest_name, &graph_name)
        .await
        .map(|record| Json(record))
}

pub async fn admin_get_distance<R, V>(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<R>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
) -> Result<Json<Vec<Vec<f64>>>>
where
    R: GraphRepository,
    V: Validator,
{
    validator.validate_user(auth).await?;
    let distance = graphs.find_distance(&contest_name, &graph_name).await?;
    let n = (distance.len() as f64).sqrt() as usize;
    let mut response = vec![vec![0.; n]; n];
    for i in 0..n {
        for j in 0..n {
            response[i][j] = distance[i * n + j];
        }
    }
    Ok(Json(response))
}

pub async fn admin_put<R, V>(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<R>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
    Json(graph): Json<GraphData>,
) -> Result<Json<Graph>>
where
    R: GraphRepository,
    V: Validator,
{
    validator.validate_user(auth).await?;
    let distance = compute_distance(&graph);
    let graph = UpsertGraph {
        contest_name,
        graph_name,
        content: graph,
        distance,
    };
    graphs.save(graph).await.map(|record| Json(record))
}
