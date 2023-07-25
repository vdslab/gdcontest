use api::{handlers, repositories::*};
use axum::{routing::get, Extension, Router};
use sqlx::PgPool;
use std::{env, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let app = Router::new()
        .route("/contests", get(handlers::list_contests))
        .route(
            "/contests/:contest_name",
            get(handlers::get_contest).put(handlers::put_contest),
        )
        .route("/contests/:contest_name/graphs", get(handlers::list_graphs))
        .route(
            "/contests/:contest_name/graphs/:graph_name",
            get(handlers::get_graph).put(handlers::put_graph),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/content",
            get(handlers::get_graph_content),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/submissions",
            get(handlers::list_submissions).post(handlers::post_submission),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/users/:user_id/submissions",
            get(handlers::list_user_submissions),
        )
        .route("/submissions/:submission_id", get(handlers::get_submission))
        .layer(cors)
        .layer(Extension(Arc::new(ContestRepository::new(pool.clone()))))
        .layer(Extension(Arc::new(GraphRepository::new(pool.clone()))))
        .layer(Extension(Arc::new(SubmissionRepository::new(pool.clone()))))
        .layer(Extension(Arc::new(AdminUserRepository::new(pool.clone()))));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
