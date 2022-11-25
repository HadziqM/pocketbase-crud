use reqwest;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Items {
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Length {
    total_items: Option<i32>,
    status: Option<i32>,
    error: Option<i32>,
    items: Option<Vec<Items>>,
}
pub struct Collection {
    pub(crate) host: String,
    pub(crate) port: i32,
    pub(crate) collection: String,
}
impl Collection {
    fn construct_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers
    }
    fn url_struct(&self) -> String {
        format!(
            "{}:{}/api/collections/{}/records",
            &self.host, &self.port, &self.collection
        )
    }
    #[tokio::main]
    pub async fn list(&self, param: Option<String>) -> String {
        let mut url = String::new();
        if param.is_some() {
            url.push_str(&[&self.url_struct(), "?", &param.unwrap()].concat());
        } else {
            url.push_str(&self.url_struct());
        };
        let client = reqwest::Client::new();
        match client.get(&url).send().await {
            Ok(result) => result.text().await.expect("no response message"),
            Err(_error) => String::from("{\"error\":400}"),
        }
    }
    #[tokio::main]
    pub async fn select(&self, id: String) -> String {
        let url = [&self.url_struct(), "/", &id].concat();
        let client = reqwest::Client::new();
        match client.get(&url).send().await {
            Ok(result) => result.text().await.expect("no response message"),
            Err(_error) => String::from("{\"error\":400}"),
        }
    }
    #[tokio::main]
    pub async fn create(&self, data: String) -> String {
        let client = reqwest::Client::new();
        match client
            .post(&self.url_struct())
            .headers(self.construct_headers())
            .body(data)
            .send()
            .await
        {
            Ok(result) => result.text().await.expect("no response message"),
            Err(_error) => String::from("{\"error\":400}"),
        }
    }
    #[tokio::main]
    pub async fn update(&self, id: String, data: String) -> String {
        let url = [&self.url_struct(), "/", &id].concat();
        let client = reqwest::Client::new();
        match client
            .patch(&url)
            .headers(self.construct_headers())
            .body(data)
            .send()
            .await
        {
            Ok(result) => result.text().await.expect("no response message"),
            Err(_error) => String::from("{\"error\":400}"),
        }
    }
    #[tokio::main]
    pub async fn delete(&self, id: String) -> String {
        let url = [&self.url_struct(), "/", &id].concat();
        let client = reqwest::Client::new();
        match client
            .delete(&url)
            .headers(self.construct_headers())
            .send()
            .await
        {
            Ok(result) => result.text().await.expect("no response message"),
            Err(_error) => String::from("{\"error\":400}"),
        }
    }
    pub async fn list_all(&self, param: Option<String>) -> String {
        let result = &self.list(Some(String::from("perPage=1")));
        let now: Length = serde_json::from_str(result).unwrap();
        if now.error.is_some() {
            return String::from("{\"error\":400}");
        } else if now.status.is_some() {
            return String::from("{\"status\":400}");
        } else {
            match param {
                Some(e) => self.list(Some(format!("perPage={}&{}", now.total_items.unwrap(), &e))),
                None => self.list(Some(format!("perPage={}", now.total_items.unwrap()))),
            }
        }
    }

    pub async fn delete_all(&self) -> String {
        let listed: Length = serde_json::from_str(&self.list_all(None).await).unwrap();
        if listed.error.is_some() {
            return String::from("{\"error\":400}");
        } else if listed.status.is_some() {
            return String::from("{\"status\":400}");
        } else {
            for i in listed.items.unwrap() {
                self.delete(i.id);
            }
            self.list(None)
        }
    }
    pub async fn update_or_create(&self, id: String, data: String) -> String {
        let listed: Length = serde_json::from_str(&self.update(id, String::from(&data))).unwrap();
        if listed.error.is_some() {
            return String::from("{\"error\":400}");
        } else if listed.status.is_some() {
            self.create(String::from(&data))
        } else {
            serde_json::to_string(&listed).unwrap()
        }
    }
}
