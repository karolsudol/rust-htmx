
 mod templates;
 mod error;
 mod db;

 use error::AppError;

 use templates::Index;

 use axum::{
     routing::get,
     Router,
 };

 use db::get_timings;



 async fn index() -> Result<Index, AppError> {
    return Ok(Index { timings: get_timings().await });
}
 
 
#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .route("/", get(index))
    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}