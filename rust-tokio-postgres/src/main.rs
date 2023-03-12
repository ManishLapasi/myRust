#![allow(unused)]
#![allow(non_snake_case)]

use tokio_postgres::{NoTls, Error, Client};
use axum::{Router, routing::get, extract::Path};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::rec_funcs::getres;

pub mod rec_funcs;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
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
    // Connect to the database.
    let (client, connection) =
    tokio_postgres::connect("connection string", NoTls).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let track_id = String::from("2f9NLCoIaiIn7rZnH9mdir");
    let ans = getres(&client, track_id).await;
    return ans;
}