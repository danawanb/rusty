use axum:: {
    extract::State, http::StatusCode, response::{Html, IntoResponse, Response}, routing::{get, post}, Form, Json, Router
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};


pub struct AppState {
    db: MySqlPool,

}

#[tokio::main]
async fn main() {
    let pool = MySqlPoolOptions::new()
        .max_connections(100)
        .connect("mysql://root:@localhost:3306/test").await.unwrap();


    let app = Router::new()
        .route("/api/healthchecker", get(get_foo))
        .route("/bar", get(foo_bar))
        .route("/insert_user", get(show_form))
        .route("/do_insert", post(create_user))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    // run our app with hyper, listening globally on port 3000
    
    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_foo() {

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

// #[debug_handler]
// fn insert_user(Form(user): Form<User>) -> (StatusCode, Json<User>) {
//     (StatusCode::CREATED, Json(payload))
// }

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `User` type
    Json(payload): Json<User>,
) -> (StatusCode, Json<User>) {
    println!("User: {:?}", payload);
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(payload))
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
            <head></head>
            <body>
                <form action="/do_insert" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
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