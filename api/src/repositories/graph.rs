use crate::{
    error::{ApiError, Result},
    models::*,
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

    pub async fn find_all(&self, contest_name: &str) -> Result<Vec<Graph>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(Graph, "sql/graphs/find_all.sql", contest_name)
            .fetch_all(&mut conn)
            .await
            .map_err(|_| ApiError::Unknown("error".into()))
    }
}

pub struct AdminGraphRepository {
    pool: PgPool,
}

impl AdminGraphRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find(&self, contest_name: &str, graph_name: &str) -> Result<AdminGraph> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            AdminGraph,
            "sql/graphs/admin_find.sql",
            contest_name,
            graph_name
        )
        .fetch_one(&mut conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
            _ => ApiError::Unknown("error".into()),
        })
    }

    pub async fn find_content(&self, contest_name: &str, graph_name: &str) -> Result<GraphData> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            GraphContent,
            "sql/graphs/admin_find_content.sql",
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
    ) -> Result<DistanceData> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            GraphDistance,
            "sql/graphs/admin_find_distance.sql",
            contest_name,
            graph_name
        )
        .fetch_one(&mut conn)
        .await
        .map(|record| record.distance)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
            _ => ApiError::Unknown("error".into()),
        })
    }

    pub async fn find_all(&self, contest_name: &str) -> Result<Vec<AdminGraph>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(AdminGraph, "sql/graphs/admin_find_all.sql", contest_name)
            .fetch_all(&mut conn)
            .await
            .map_err(|_| ApiError::Unknown("error".into()))
    }

    pub async fn save(&self, graph: &AdminUpsertGraph) -> Result<AdminGraph> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            AdminGraph,
            "sql/graphs/admin_save.sql",
            graph.contest_name,
            graph.graph_name,
            json!(graph.content),
            graph.distance,
        )
        .fetch_one(&mut conn)
        .await
        .map_err(|e| ApiError::Unknown(e.to_string()))
    }

    pub async fn delete(&self, contest_name: &str, graph_name: &str) -> Result<()> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file!("sql/graphs/admin_delete.sql", contest_name, graph_name,)
            .execute(&mut conn)
            .await
            .map(|_| ())
            .map_err(|_| ApiError::Unknown("error".into()))
    }
}
