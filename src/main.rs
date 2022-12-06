mod crud;

use chrono::prelude::*;
// use chrono::serde::ts_seconds::serialize as to_ts;
// use reqwest;
use serde::{Deserialize, Serialize};

// #[derive(Serialize)]
// struct CreateUser {
//     username: String,
//     #[serde(serialize_with = "to_ts")]
//     created_at: DateTime<Utc>,
//     field: String,
//     money: u128,
// }

// #[derive(Serialize)]
// struct CreateUserData {
//     data: CreateUser,
// }
#[derive(Serialize)]
struct CreateUser {
    username: String,
    field: String,
    money: u128,
}

#[derive(Deserialize)]
struct ForVect {
    // collectionName: String,
    // field: String,
    id: String,
    // money: u128,
    // updated: String,
    // username: String,
}

// #[derive(Deserialize)]
// struct User {
//     items: Vec<For_vect>,
// }

fn main() {
    let user = crud::Collection {
        host: "http://127.0.0.1".to_string(),
        port: 8090,
        collection: "test".to_string(),
    };
    // let mydata: CreateUser = CreateUser {
    //     username: "now2".to_string(),
    //     field: "Pro".to_string(),
    //     money: 100,
    // };
    // let data: String = serde_json::to_string(&mydata).unwrap();
    // let created_data: ForVect = serde_json::from_str(&created).unwrap();
    // println!("example of list {}", user.list(None));
    let now = Utc::now().format("%+");
    let data = ["{\"field\":\"", format!("{}", now).as_ref(), "\"}"].concat();
    let created = user.create(data);
    println!("example of create {}", &created);
    // println!("{}", created_data.id);
    // println!(
    //     "example of update{}",
    //     user.update(
    //         String::from(&created_data.id),
    //         String::from(r#"{"money":1234}"#)
    //     )
    // );
    // println!(
    //     "example of select {}",
    //     user.select(String::from(&created_data.id))
    // );
    // println!(
    //     "example of delete {}",
    //     user.delete(String::from(&created_data.id))
    // );
}
