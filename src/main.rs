use std::collections::HashMap;
use std::future::Future;

use std::pin::Pin;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use rocket::response::Responder;

use tokio::io::AsyncReadExt;

use rocket::fs::NamedFile;
use std::path::PathBuf;



use rocket::http::{Status, ContentType};
use rocket::response::content::RawJson;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::serde::json::serde_json::json;
use rocket::{State, routes, get, post, delete, Response};

use rocket_cors::{AllowedOrigins, CorsOptions};

use serde::{Deserialize, Serialize};

type Todos = Arc<Mutex<HashMap<String, Vec<String>>>>;

#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    todo: String,
}

#[allow(unused_must_use)]
#[rocket::main]
async fn main() {
    let todos: Todos = Arc::new(Mutex::new(HashMap::new()));

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&[
            "http://localhost:5002",
            "http://localhost:8000",
            "http://127.0.0.1:8000",
            "https://chat.openai.com",
        ]))
        .allowed_methods(["Get", "Post", "Delete"]
            .iter()
            .map(|s| FromStr::from_str(s).unwrap())
            .collect()
        )
        .to_cors()
        .expect("failed to create CORS");

    let built = rocket::build()
        .manage(todos)
        .attach(cors)
        .mount(
            "/",
            routes![
                plugin_manifest,
                get_todos,
                get_todo_user,
                add_todo,
                delete_todo,
                plugin_logo,
                openapi_spec,
                healthcheck,
            ],
        );
    built
        .launch()
        .await;
}

// at "/" respond with "I'm a todo plugin for chatgpt!"
#[get("/hello")]
async fn healthcheck() -> &'static str {
    "I'm a todo plugin for chatgpt!"
}

#[get("/todos")]
async fn get_todos(todos: &State<Todos>) -> Json<HashMap<String, Vec<String>>> {
    Json(todos.lock().unwrap().clone())
}

#[get("/todos/<username>")]
async fn get_todo_user(username: String, todos: &State<Todos>) -> Json<Vec<String>> {
    let todos_map = todos.lock().unwrap();
    Json(todos_map.get(&username).unwrap_or(&vec![]).clone())
}

#[post("/todos/<username>", format = "json", data = "<todo_item>")]
async fn add_todo(
    username: String,
    todo_item: Json<TodoItem>,
    todos: &State<Todos>,
) -> Json<Value> {
    let mut todos_map = todos.lock().unwrap();
    todos_map
        .entry(username)
        .or_insert_with(Vec::new)
        .push(todo_item.todo.clone());
    Json(json!({"status": "success"}))
}

#[delete("/todos/<username>", format = "json", data = "<todo_idx>")]
async fn delete_todo(username: String, todo_idx: Json<usize>, todos: &State<Todos>) -> Json<Value> {
    let mut todos_map = todos.lock().unwrap();
    if let Some(user_todos) = todos_map.get_mut(&username) {
        let inner_idx = todo_idx.into_inner();
        if let Some(_) = user_todos.get(inner_idx) {
            user_todos.remove(inner_idx);
        }
    }
    Json(json!({"status": "success"}))
}

#[get("/logo.png")]
async fn plugin_logo() -> std::result::Result<NamedFile, Status> {
    let path = PathBuf::from("logo.png");
    NamedFile::open(path)
        .await
        .map_err(|_| Status::NotFound)
}

#[get("/.well-known/ai-plugin.json")]
async fn plugin_manifest() -> std::result::Result<RawJson<String>, Status> {
    println!("plugin_manifest");
    let mut file = tokio::fs::File::open("manifest.json")
        .await
        .map_err(|_| Status::NotFound)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .await
        .map_err(|_| Status::InternalServerError)?;
    let host =
        std::env::var("PLUGIN_HOSTNAME").unwrap_or_else(|_| "https://localhost:5002".to_string());
    buffer = buffer.replace("PLUGIN_HOSTNAME", &host);
    Ok(RawJson(buffer))
}

// let's just build a custom responder for openapi.yaml


pub struct YamlResponder {
    pub path: String,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for YamlResponder {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let path = self.path.clone();
        let response = async move {
            let mut file = match tokio::fs::File::open(&path).await {
                Ok(file) => file,
                Err(_) => return Err(rocket::http::Status::NotFound),
            };

            let mut buffer = String::new();
            if let Err(_) = file.read_to_string(&mut buffer).await {
                return Err(rocket::http::Status::InternalServerError);
            }

            let host = std::env::var("PLUGIN_HOSTNAME")
                .unwrap_or_else(|_| "https://localhost:5002".to_string());
            buffer = buffer.replace("PLUGIN_HOSTNAME", &host);

            Response::build()
                .header(ContentType::new("text", "yaml"))
                .sized_body(buffer.len(), std::io::Cursor::new(buffer))
                .ok()
        };

        YamlResponseFuture(Box::pin(response)).into()
    }
}

pub struct YamlResponseFuture(Pin<Box<dyn Future<Output = rocket::response::Result<'static>> + Send>>);

impl Into<rocket::response::Result<'static>> for YamlResponseFuture {
    fn into(self) -> rocket::response::Result<'static> {
        futures::executor::block_on(self.0)
    }
}

#[get("/openapi.yaml")]
async fn openapi_spec() -> YamlResponder {
    YamlResponder {
        path: "openapi.yaml".to_string(),
    }
}
