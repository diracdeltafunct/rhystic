use axum::{Router, routing::get};



pub async fn start_dev_server(){
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}