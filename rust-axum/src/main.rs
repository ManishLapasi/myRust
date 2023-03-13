use axum::{Router, routing::get, extract::Path};

#[tokio::main] 
async fn main() {
    let app: Router = Router::new()
        .route("/",get(root))
        .route("/get_recomms/:id",get(get_recomms));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn root() -> String{
    return String::from("hello world!");
}

async fn get_recomms(Path(id): Path<String>) -> String{
    return String::from(id);
}