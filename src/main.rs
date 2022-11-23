use std::process::id;

use chrono::prelude::*;
use chrono::serde::ts_seconds::serialize as to_ts;
use reqwest;
use serde::{Deserialize, Serialize};

// #[derive(Serialize)]
// struct CreateUser {
//     username: String,
//     #[serde(serialize_with = "to_ts")]
//     created_at: DateTime<Utc>,
//     field: String,
//     money: u128,
// }

#[derive(Serialize)]
struct CreateUser {
    username: String,
    field: String,
    money: u128,
}
#[derive(Serialize)]
struct CreateUserData {
    data: CreateUser,
}

#[derive(Deserialize)]
struct For_vect {
    collectionName: String,
    field: String,
    id: String,
    money: u128,
    updated: String,
    username: String,
}

#[derive(Deserialize)]
struct User {
    items: Vec<For_vect>,
}

#[tokio::main]
async fn get_user(url: String) -> User {
    let mut some_string = String::from(&url);
    some_string.push_str("/api/collections/user/records");
    let client = reqwest::Client::new();
    let resp = client
        .get(&some_string)
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .expect("failed");
    return resp;
}

fn construct_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

#[tokio::main]
async fn create_user(user: &str, url: String) -> String {
    let mut some_string = String::from(&url);
    some_string.push_str("/api/collections/user/records");
    let client = reqwest::Client::new();
    let idk = client
        .post(&some_string)
        .body(String::from(user))
        .headers(construct_headers())
        .send()
        .await
        .expect("fail to post")
        .text()
        .await
        .expect("failed to post");
    return idk;
}

fn main() {
    let idk = get_user(String::from("http://192.168.0.110:8090"));
    let mydata = CreateUser {
        username: "Please Work!".to_string(),
        field: "Pro".to_string(),
        money: 10000,
    };
    // let newdata = CreateUserData { data: mydata };
    let json = serde_json::to_string(&mydata).unwrap();
    println!("{}", json);
    println!("{}", idk.items[0].username);
    println!(
        "{}",
        create_user(&json, "http://192.168.0.110:8090".to_string())
    )
}
