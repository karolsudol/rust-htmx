
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
        .route("/", get(index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();

}