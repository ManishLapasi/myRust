#![allow(unused)]
#![allow(non_snake_case)]

use axum::{Router, routing::get, extract::Path, extract::Extension};
use tokio_postgres::{NoTls, Error, Client, Config};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tower_http::cors::{Any, CorsLayer};
use dotenv::dotenv;
use crate::rec_funcs::getres;

pub mod rec_funcs;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() {

    // load env vars
    dotenv().ok();

    let mut pg_config = tokio_postgres::Config::new();
    let host = std::env::var("HOST").expect("HOST for DB not set");
    let user = std::env::var("USER").expect("USER for DB not set");
    let password = std::env::var("PASSWORD").expect("PASSWORD for DB not set");
    let dbname = std::env::var("DBNAME").expect("DBNAME for DB not set");
    pg_config.host(&host).user(&user).port(5432).password(&password).dbname(&dbname);

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();

    let cors = CorsLayer::new().allow_origin(Any);

    let app: Router = Router::new()
        .route("/",get(root))
        .route("/get_recomms/:id",get(get_recomms))
        .layer(cors)
        .layer(Extension(pool));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
   
}

async fn root() -> String{
    return String::from("hello world!");
}

async fn get_recomms(Extension(pool): Extension<Pool>,Path(id): Path<String>) -> String{
    // Connect to the database.
    /*
    Some verification code I wrote to verify db connections !
    for i in 1..10 {
        let mut client = pool.get().await.unwrap();
        let stmt = client.prepare_cached("SELECT 1 + $1").await.unwrap();
        let rows = client.query(&stmt, &[&i]).await.unwrap();
        let value: i32 = rows[0].get(0);
        assert_eq!(value, i + 1);
        println!("{value}");
    }
    */

    // The next line is an example of a track id to query with! Uncomment and replace "id" with "track_id" in the call to getres
    //let track_id = String::from("2f9NLCoIaiIn7rZnH9mdir");
    let client = pool.get().await.unwrap();
    let ans = getres(&client, id).await;
    return ans;
}