use crate::{
    error::{ApiError, Result},
    models::{DistanceData, Graph, GraphContent, GraphData, GraphDistance, GraphMeta, UpsertGraph},
};
use serde_json::json;
use sqlx::PgPool;

pub struct GraphRepository {
    pool: PgPool,
}

impl GraphRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find(&self, contest_name: &str, graph_name: &str) -> Result<Graph> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(Graph, "sql/graphs/find.sql", contest_name, graph_name)
            .fetch_one(&mut conn)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
                _ => ApiError::Unknown("error".into()),
            })
    }

    pub async fn find_meta(&self, contest_name: &str, graph_name: &str) -> Result<GraphMeta> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            GraphMeta,
            "sql/graphs/find_meta.sql",
            contest_name,
            graph_name
        )
        .fetch_one(&mut conn)
        .await
        .map(|record| record)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
            _ => ApiError::Unknown("error".into()),
        })
    }

    pub async fn find_content(&self, contest_name: &str, graph_name: &str) -> Result<GraphData> {
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

    pub async fn find_distance(
        &self,
        contest_name: &str,
        graph_name: &str,
    ) -> Result<Vec<Vec<f64>>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            GraphDistance,
            "sql/graphs/find_distance.sql",
            contest_name,
            graph_name
        )
        .fetch_one(&mut conn)
        .await
        .map(|record| record.distance.0)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
            _ => ApiError::Unknown("error".into()),
        })
    }

    pub async fn find_all(&self, contest_name: &str) -> Result<Vec<GraphMeta>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(GraphMeta, "sql/graphs/find_all.sql", contest_name)
            .fetch_all(&mut conn)
            .await
            .map_err(|_| ApiError::Unknown("error".into()))
    }

    pub async fn save(&self, graph: &UpsertGraph) -> Result<UpsertGraph> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            UpsertGraph,
            "sql/graphs/save.sql",
            graph.contest_name,
            graph.graph_name,
            json!(graph.content),
            json!(graph.distance)
        )
        .fetch_one(&mut conn)
        .await
        .map_err(|_| ApiError::Unknown("error".into()))
    }
}
