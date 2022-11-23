mod crud;

// use chrono::prelude::*;
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
#[derive(Serialize)]
struct Money {
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
        host: "http://192.168.0.110".to_string(),
        port: 8090,
        collection: "user".to_string(),
    };
    let mydata: CreateUser = CreateUser {
        username: "now2".to_string(),
        field: "Pro".to_string(),
        money: 100,
    };
    let data: String = serde_json::to_string(&mydata).unwrap();
    let mymoney: Money = Money { money: 123 };
    let money: String = serde_json::to_string(&mymoney).unwrap();
    let created = user.create(data);
    let created_data: ForVect = serde_json::from_str(&created).unwrap();
    println!("example of create {}", &created);
    println!("example of list {}", user.list());
    println!("{}", created_data.id);
    println!(
        "example of update{}",
        user.update(String::from(&created_data.id), money)
    );
    println!(
        "example of select {}",
        user.select(String::from(&created_data.id))
    );
    println!(
        "example of delete {}",
        user.delete(String::from(&created_data.id))
    );
}
