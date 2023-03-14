mod handlers;
mod repositories;
use hyper::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer, Origin};

use crate::repositories::{TodoRepository, TodoRepositoryForDb};
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use handlers::{create_todo, delete_todo, find_all, find_todo, update_todo};
use sqlx::PgPool;
use std::net::SocketAddr;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("into".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = &env::var("DATABASE_URL").expect("undifind [DATABASE_URL]");
    tracing::debug!("start connect database");
    let pool = PgPool::connect(database_url).await.expect("fail DB");
    let repository = TodoRepositoryForDb::new(pool.clone());
    let app = create_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3010));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>).get(find_all::<T>))
        .route(
            "/todos/:id",
            get(find_todo::<T>)
                .delete(delete_todo::<T>)
                .patch(update_todo::<T>),
        )
        .layer(Extension(Arc::new(repository)))
        .layer(
            CorsLayer::new()
                .allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()))
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
}

async fn root() -> &'static str {
    "Hello world"
}
