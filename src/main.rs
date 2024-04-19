use axum:: {
    extract::{Path, State}, http::StatusCode, response::{Html, IntoResponse, Response}, routing::{get, post, MethodRouter}, Form, Json, Router
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use rand::Rng;


pub struct AppState {
    db: MySqlPool,

}

#[tokio::main]
async fn main() {
    let pool = MySqlPoolOptions::new()
        .max_connections(100)
        .connect("mysql://root:@localhost:3306/test").await.unwrap();


    let app = Router::new()
        // .route("/api/healthchecker", get(get_foo))
        .route("/bar", get(foo_bar))
        .route("/insert_user", get(show_form))
        .route("/do_insert", post(create_user))
        .route("/do_insert_2", post(accept_form))
        .route("/hello", get(get_name))
        .route("/get_email_by_id/:id", get(get_email_by_id))
        .with_state(Arc::new(AppState { db: pool.clone() }))
        .merge(using_serve_file_from_a_route());

    // run our app with hyper, listening globally on port 3000
    
    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


#[derive(sqlx::FromRow, Serialize, Debug, Deserialize)]
#[allow(dead_code)]
struct User {
    user : String,
    email : String
}


async fn foo_bar(
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {
    
    let result = sqlx::query_as::<_, User>("SELECT user, email FROM users LIMIT 1")
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(x) => Json(json!(x)),
        Err(y) => Json(json!({
            "error" : true,
            "message" : y.to_string(),
        })),
    }
}

fn using_serve_file_from_a_route() -> Router {
    // `ServeDir` allows setting a fallback if an asset is not found
    // so with this `GET /assets/doesnt-exist.jpg` will return `index.html`
    // rather than a 404
    let serve_dir = ServeDir::new("./svelte/build").not_found_service(ServeFile::new("./svelte/build/index.html"));

    Router::new()
       // .route("/foo", get(|| async { "Hi from /foo" }))
        .nest_service("/svelte", serve_dir.clone())
        .fallback_service(serve_dir)
}
async fn get_random_color() -> impl IntoResponse {

    let mut rng = rand::thread_rng();
    let color: String = format!("#{:06x}", rng.gen::<u32>());

    Json(json!({
        "color": color
    }))
}

async fn create_user(  State(data): State<Arc<AppState>>, Json(payload): Json<User>) -> (StatusCode, Json<String>) {
    println!("User: {:?}", payload);
    let res = sqlx::query(r#"INSERT INTO users (user, email) VALUES (?, ?)"#)
        .bind(&payload.user)
        .bind(&payload.email)
        .execute(&data.db)
        .await;
    match res {
        Ok(_) => (StatusCode::CREATED, Json("Berhasil Insert User".to_owned())),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
    }
    
}

async fn get_email_by_id(Path::<i32>(id): Path<i32>, State(data): State<Arc<AppState>>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, User>("SELECT user, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&data.db)
        .await;

    match result {
        Ok(x) => Json(json!(x)),
        Err(y) => Json(json!({
            "error" : true,
            "message" : y.to_string(),
        })),
    }
}

async fn get_name(Path::<String>(name):Path<String>) -> impl IntoResponse {
    Json(json!(name))
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
            
            </head>
            <body>
                <form action="/do_insert_2" method="post">
                    <label for="user">
                        Enter your name:
                        <input type="text" name="user" id="user">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email" id="user">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

async fn accept_form(Form(user): Form<User>) -> (StatusCode, Json<Responses<User>>){
    println!("{:?}", user);
    let res = Responses {
        data : user
    };
    
    (StatusCode::CREATED, Json(res))
}


#[derive(Serialize, Debug, Deserialize)]
struct Responses <T> {
    data : T,
}

fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}