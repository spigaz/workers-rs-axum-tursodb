use axum::{
    Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

#[allow(deprecated)]
use libsql_client::Client;
use serde::{Deserialize, Serialize};
use tower_service::Service;
use worker::*;

fn router(env: Env) -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /todos` goes to `create_todo`
        .route("/todos", get(get_todos))
        // `POST /todos` goes to `create_todo`
        .route("/todos", post(create_todo))
        .layer(Extension(env))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    tracing_subscriber::fmt::init();

    let conn = connection(&env).await;

    #[allow(deprecated)]
    let _ = conn
        .execute("CREATE TABLE IF NOT EXISTS todos(task varchar non null)")
        .await;

    tracing::debug!("starting");

    Ok(router(env).call(req).await?)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, Axum! ❤︎ Turso"
}

// Initializes a database connection
#[allow(deprecated)]
async fn connection(env: &Env) -> Client {
    #[allow(deprecated)]
    // Uses secrets "LIBSQL_CLIENT_URL" and "LIBSQL_CLIENT_TOKEN"
    Client::from_workers_env(env).unwrap()
}

// Gets all tasks from the todo table
#[worker::send]
async fn get_todos(Extension(env): Extension<Env>) -> impl IntoResponse {
    let conn = connection(&env).await;

    #[allow(deprecated)]
    let results = conn.execute("SELECT * FROM todos").await.unwrap().rows;

    let mut todos: Vec<Todo> = Vec::new();

    for row in results {
        let todo: Todo = Todo {
            #[allow(deprecated)]
            task: row.values.get(0).unwrap().to_string(),
        };
        todos.push(todo);
    }

    (StatusCode::OK, Json(todos))
}

// Creates a new task in the todo table
#[worker::send]
async fn create_todo(
    Extension(env): Extension<Env>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = Todo { task: payload.task };

    let conn = connection(&env).await;

    #[allow(deprecated)] // NOT SAFE!!! - WAS "INSERT into todos values (?1)", params![todo.task.clone()]
    let _ = conn
        .execute(format!("INSERT into todos values ({})", todo.task))
        .await;

    (StatusCode::CREATED, Json(todo))
}

// the struct for a todo item
#[derive(Serialize)]
struct Todo {
    task: String,
}

// the input to the create_todos handler
#[derive(Deserialize, Serialize, Debug)]
struct CreateTodo {
    task: String,
}
