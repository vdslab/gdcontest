use api::{handlers, repositories::*};
use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};
use dotenv::dotenv;
use sqlx::PgPool;
use std::{env, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let admin_routes = Router::new()
        .route("/contests", get(handlers::contest::admin_list))
        .route("/contests/:contest_name", get(handlers::contest::admin_get))
        .route("/contests/:contest_name", put(handlers::contest::admin_put))
        .route(
            "/contests/:contest_name",
            delete(handlers::contest::admin_delete),
        )
        .route(
            "/contests/:contest_name/graphs",
            get(handlers::graph::admin_list),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name",
            get(handlers::graph::admin_get),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name",
            put(handlers::graph::admin_put),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/content",
            get(handlers::graph::admin_get_content),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/distance",
            get(handlers::graph::admin_get_distance),
        );
    let app = Router::new()
        .route("/contests", get(handlers::contest::list))
        .route("/contests/:contest_name", get(handlers::contest::get))
        .route("/contests/:contest_name/graphs", get(handlers::graph::list))
        .route(
            "/contests/:contest_name/graphs/:graph_name",
            get(handlers::graph::get),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/content",
            get(handlers::graph::get_content),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/submissions",
            get(handlers::submission::list),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/submissions",
            post(handlers::submission::post),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/standings",
            get(handlers::submission::list_standings),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/users/:user_id/submissions",
            get(handlers::submission::list_by_user),
        )
        .route(
            "/submissions/:submission_id",
            get(handlers::submission::get),
        )
        .nest("/admin", admin_routes)
        .layer(cors)
        .layer(Extension(Arc::new(ContestRepository::new(pool.clone()))))
        .layer(Extension(Arc::new(GraphRepository::new(pool.clone()))))
        .layer(Extension(Arc::new(SubmissionRepository::new(pool.clone()))))
        .layer(Extension(Arc::new(AdminContestRepository::new(
            pool.clone(),
        ))))
        .layer(Extension(Arc::new(AdminGraphRepository::new(pool.clone()))));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
