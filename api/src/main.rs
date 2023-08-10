use api::{auth::ValidatorImpl, handlers, repositories::*};
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
        .route(
            "/contests",
            get(handlers::contest::admin_list::<ValidatorImpl>),
        )
        .route(
            "/contests/:contest_name",
            get(handlers::contest::admin_get::<ValidatorImpl>),
        )
        .route(
            "/contests/:contest_name",
            put(handlers::contest::admin_put::<ValidatorImpl>),
        )
        .route(
            "/contests/:contest_name",
            delete(handlers::contest::admin_delete::<ValidatorImpl>),
        )
        .route(
            "/contests/:contest_name/graphs",
            get(handlers::graph::admin_list::<GraphRepositoryForDB, ValidatorImpl>),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name",
            get(handlers::graph::admin_get::<GraphRepositoryForDB, ValidatorImpl>),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name",
            put(handlers::graph::admin_put::<GraphRepositoryForDB, ValidatorImpl>),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/content",
            get(handlers::graph::admin_get_content::<GraphRepositoryForDB, ValidatorImpl>),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/distance",
            get(handlers::graph::admin_get_distance::<GraphRepositoryForDB, ValidatorImpl>),
        );
    let app = Router::new()
        .route("/contests", get(handlers::contest::list))
        .route("/contests/:contest_name", get(handlers::contest::get))
        .route(
            "/contests/:contest_name/graphs",
            get(handlers::graph::list::<GraphRepositoryForDB>),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name",
            get(handlers::graph::get::<GraphRepositoryForDB>),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/content",
            get(handlers::graph::get_content::<GraphRepositoryForDB>),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/submissions",
            get(handlers::submission::list::<SubmissionRepositoryForDB>),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/submissions",
            post(
                handlers::submission::post::<
                    SubmissionRepositoryForDB,
                    GraphRepositoryForDB,
                    ValidatorImpl,
                >,
            ),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/standings",
            get(handlers::submission::list_standings::<SubmissionRepositoryForDB>),
        )
        .route(
            "/contests/:contest_name/graphs/:graph_name/users/:user_id/submissions",
            get(handlers::submission::list_by_user::<SubmissionRepositoryForDB>),
        )
        .route(
            "/submissions/:submission_id",
            get(handlers::submission::get::<SubmissionRepositoryForDB>),
        )
        .nest("/admin", admin_routes)
        .layer(cors)
        .layer(Extension(Arc::new(ContestRepository::new(pool.clone()))))
        .layer(Extension(Arc::new(AdminContestRepository::new(
            pool.clone(),
        ))))
        .layer(Extension(Arc::new(GraphRepositoryForDB::new(pool.clone()))))
        .layer(Extension(Arc::new(SubmissionRepositoryForDB::new(
            pool.clone(),
        ))))
        .layer(Extension(Arc::new(UserRepositoryForDB::new(pool.clone()))))
        .layer(Extension(Arc::new(ValidatorImpl {
            user: env::var("AUTH_USER").expect("AUTH_USER must be set"),
            password: env::var("AUTH_PASSWORD").expect("AUTH_PASSWORD must be set"),
        })));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
