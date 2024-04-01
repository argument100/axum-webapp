use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Mydata {
    name: String,
    mail: String,
    age: u32,
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", axum::routing::get(handler_top))
        .route("/usr/:id/:user", axum::routing::get(handler_param))
        .route("/qry", axum::routing::get(handler_query))
        .route("/json/:id", axum::routing::get(handler_json));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_top() -> String {
    "Hello, World!".to_string()
}

async fn handler_param(axum::extract::Path((id, user)): axum::extract::Path<(usize, String)>) -> String {
    format!("User ID: {}. name: {}.", id, user)
}

async fn handler_query(axum::extract::Query(params):
                       axum::extract::Query<std::collections::HashMap<String, String>>) -> String {
    format!("id: {}, name: {}", params["id"], params["name"])
}

async fn handler_json(axum::extract::Path(id): axum::extract::Path<usize>) -> axum::Json<serde_json::Value> {
    let data:[Mydata;3] = [
        Mydata {name: String::from("Taro"), mail: String::from("taro@yamada"), age: 39},
        Mydata {name: String::from("Hanako"), mail: String::from("hanako@flower"), age: 28},
        Mydata {name: String::from("Sachiko"), mail: String::from("sachiko@happy"), age: 17},
    ];
    let item = &data[id];
    let data = serde_json::json!(item);
    axum::Json(data)
}
