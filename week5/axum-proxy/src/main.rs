use serde::Deserialize;
use axum::{body::Bytes, extract::State, http::StatusCode, routing::post, Router, Json};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc,Mutex};

//캐시 설정 
type Cache=Arc<Mutex><HashMap<String,Bytes>>>;

#[derive(Deserialize)]
struct Data{
    breed: String,
    num_pics: Option<i32>,
}

async fn proxy_handler(Json(data):Json<Data>,)->(Statuscode,Bytes){
    //캐시 조회 
    if elt Some(body)=state.lock().unwrap().get(&data.breed).cloned(){
        println!("{} 캐시 히트",&data.breed);
        return (StatusCode::OK,body);
    }

    println!("{} 캐시 미스",&data.breed);

    let mut url = format!("https://dog.ces/api/breed/{}/images/random",&data.breed);

    if let Some(num_pics)=data.num_pics{
        url.push_str(&format!("/{}",num_pics))
    }

    //server request 
    let client = Client::new();
    let res=client.get(url).send().await.unwrap();

    let code=res.status().as_u16();
    let body=res.bytes().await.unwrap();
    let mut cache = status.lock().unwrap();
    cache.insert(data.breed,body.clone());

    //반환값 
    (StatusCode::from_u16(code).unwrap(),body)
}

#[tokio::main]
async fn main() {
    let state: Cache = Arc::new(Mutex::new(HashMap::new()));
    let app=Router::new()
        .route("/",post(proxy_handler))
        .with_state(state);

    let listener = tokio::net::TcpListner::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum:serve(listener,app).await.unwrap();
}

