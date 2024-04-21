use axum::{
    routing::get, Json, Router
};
use serde::{Deserialize, Serialize};
use reqwest::{Client, Url};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(pokesort));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Serialize)]
struct Ability {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct AbilityRes {
    results: Vec<Ability>
}   

async fn pokesort() ->  Result<Json<Vec<Ability>>, String> {
    let client = Client::new();
    let res = client
        .get(
            Url::parse(
                "http://127.0.0.1:8000/api/v2/ability/",
            )
            .expect("Error parsing URL"),
        )
        .send()
        .await;
    
    match res {
        Err(_) => Err("Error".into()),
        Ok(res) => match res.json::<AbilityRes>().await {
            Err(_) => Err("Error".into()),
            Ok(mut json) => {
                json.results.sort_by(|a,b| a.name.partial_cmp(&b.name).expect(""));
                Ok(Json(json.results))
            }
        },
    }
}
