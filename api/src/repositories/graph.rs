use crate::{
    error::{ApiError, Result},
    graph::*,
};
use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Graph {
    pub contest_name: String,
    pub graph_name: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Serialize)]
struct GraphContent {
    pub content: sqlx::types::Json<GraphData>,
}

#[derive(Serialize)]
struct GraphDistance {
    pub distance: Vec<u8>,
}

#[derive(Serialize)]
pub struct UpsertGraph {
    pub contest_name: String,
    pub graph_name: String,
    pub content: GraphData,
    pub distance: DistanceData,
}

#[async_trait]
pub trait GraphRepository: Clone + Send + Sync + 'static {
    async fn find(&self, contest_name: &str, graph_name: &str) -> Result<Graph>;
    async fn find_content(&self, contest_name: &str, graph_name: &str) -> Result<GraphData>;
    async fn find_distance(&self, contest_name: &str, graph_name: &str) -> Result<DistanceData>;
    async fn find_all(&self, contest_name: &str) -> Result<Vec<Graph>>;
    async fn delete(&self, contest_name: &str, graph_name: &str) -> Result<()>;
    async fn save(&self, graph: UpsertGraph) -> Result<Graph>;
}

#[derive(Clone)]
pub struct GraphRepositoryForDB {
    pool: PgPool,
}

impl GraphRepositoryForDB {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GraphRepository for GraphRepositoryForDB {
    async fn find(&self, contest_name: &str, graph_name: &str) -> Result<Graph> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(Graph, "sql/graphs/find.sql", contest_name, graph_name)
            .fetch_one(&mut conn)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
                _ => ApiError::Unknown("error".into()),
            })
    }

    async fn find_content(&self, contest_name: &str, graph_name: &str) -> Result<GraphData> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            GraphContent,
            "sql/graphs/find_content.sql",
            contest_name,
            graph_name
        )
        .fetch_one(&mut conn)
        .await
        .map(|record| record.content.0)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
            _ => ApiError::Unknown("error".into()),
        })
    }

    async fn find_distance(&self, contest_name: &str, graph_name: &str) -> Result<DistanceData> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            GraphDistance,
            "sql/graphs/find_distance.sql",
            contest_name,
            graph_name
        )
        .fetch_one(&mut conn)
        .await
        .map(|record| distance_from_bytes(record.distance))
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
            _ => ApiError::Unknown("error".into()),
        })
    }

    async fn find_all(&self, contest_name: &str) -> Result<Vec<Graph>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(Graph, "sql/graphs/find_all.sql", contest_name)
            .fetch_all(&mut conn)
            .await
            .map_err(|_| ApiError::Unknown("error".into()))
    }

    async fn save(&self, graph: UpsertGraph) -> Result<Graph> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            Graph,
            "sql/graphs/save.sql",
            graph.contest_name,
            graph.graph_name,
            json!(graph.content),
            distance_into_bytes(graph.distance),
        )
        .fetch_one(&mut conn)
        .await
        .map_err(|e| ApiError::Unknown(e.to_string()))
    }

    async fn delete(&self, contest_name: &str, graph_name: &str) -> Result<()> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file!("sql/graphs/delete.sql", contest_name, graph_name,)
            .execute(&mut conn)
            .await
            .map(|_| ())
            .map_err(|_| ApiError::Unknown("error".into()))
    }
}

#[derive(Clone)]
pub struct GraphRepositoryForMemory;

#[async_trait]
impl GraphRepository for GraphRepositoryForMemory {
    async fn find(&self, _contest_name: &str, _graph_name: &str) -> Result<Graph> {
        unimplemented!("unimplemented!")
    }

    async fn find_content(&self, _contest_name: &str, _graph_name: &str) -> Result<GraphData> {
        unimplemented!("unimplemented!")
    }

    async fn find_distance(&self, _contest_name: &str, _graph_name: &str) -> Result<DistanceData> {
        unimplemented!("unimplemented!")
    }

    async fn find_all(&self, _contest_name: &str) -> Result<Vec<Graph>> {
        unimplemented!("unimplemented!")
    }

    async fn delete(&self, _contest_name: &str, _graph_name: &str) -> Result<()> {
        unimplemented!("unimplemented!")
    }

    async fn save(&self, _graph: UpsertGraph) -> Result<Graph> {
        unimplemented!("unimplemented!")
    }
}

fn distance_from_bytes(bytes: Vec<u8>) -> Vec<f64> {
    unsafe {
        std::slice::from_raw_parts(
            bytes.as_ptr() as *const f64,
            bytes.len() / std::mem::size_of::<f64>(),
        )
        .to_vec()
    }
}

fn distance_into_bytes(distance: Vec<f64>) -> Vec<u8> {
    unsafe {
        std::slice::from_raw_parts(
            distance.as_ptr() as *const u8,
            distance.len() * std::mem::size_of::<f64>(),
        )
        .to_vec()
    }
}
