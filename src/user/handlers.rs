
#[derive(sqlx::FromRow, Serialize, Debug, Deserialize)]
struct User {
    id : i32,
    username : String,
    password : String
}

#[derive(sqlx::FromRow, Serialize, Debug, Deserialize, Validate)]
struct UserInsert {
    #[validate(length(min = 5,  message = "Tidak boleh kosong minimal 5 karakter"))]
    username : String,
    #[validate(length(min =5, max=20, message="Password maksimal 20 karakter minimal 5"))]
    password : String
}


#[derive(sqlx::FromRow, Serialize, Debug, Deserialize)]
struct UserRes{
    id : i32,
    username : String,
}

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Error, Json, Router};
use bcrypt::hash;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::AppState;

pub  fn  user_handlers() -> Router<Arc<AppState>> {

    Router::new()
        .route("/single", get(foo_bar))
        .route("/insert", post(insert_user))
        .route("/all", get(get_all_user))
}


#[derive(Serialize, Deserialize)]
enum ApiResponse<T> {
    OK,
    Created,
    ErrorInternal(String),
    JsonData(User),
    JsonVec(Vec<T>)
}

enum ApiError {
    BadRequest(String),
    Forbidden,
    Unauthorised,
    InternalServerError(String),
    AlreadyExist(String)
}

impl IntoResponse for ApiResponse<UserRes> {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
            Self::ErrorInternal(x) =>(StatusCode::INTERNAL_SERVER_ERROR, Json(x)).into_response(),
            Self::JsonVec(x) => (StatusCode::OK, Json(x)).into_response()
        }
    }
}

impl IntoResponse for ApiResponse<User> {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
            Self::ErrorInternal(x) =>(StatusCode::INTERNAL_SERVER_ERROR, Json(x)).into_response(),
            Self::JsonVec(x) => (StatusCode::OK, Json(x)).into_response()
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(x) => (StatusCode::BAD_REQUEST, Json(x)).into_response(),
            Self::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            Self::Unauthorised => (StatusCode::OK).into_response(),
            Self::InternalServerError(x) =>(StatusCode::INTERNAL_SERVER_ERROR, Json(x)).into_response(),
            Self::AlreadyExist(x) => {
                (StatusCode::BAD_REQUEST, Json(x)).into_response()
            }
        }
    }
}
async fn foo_bar(State(data): State<Arc<AppState>>) -> Result<ApiResponse<User>, ApiError> {
    let result = sqlx::query_as::<_, User>("SELECT id, username, password FROM users LIMIT 1")
        .fetch_one(&data.pg)
        .await;

    match result {
        Ok(x) => Ok(ApiResponse::JsonData(x)),
        Err(y) => Err(ApiError::InternalServerError(y.to_string()))
    }
}
 
async fn get_all_user(State(data): State<Arc<AppState>>) -> Result<ApiResponse<UserRes>, ApiError> {
    let res : Result<Vec<UserRes>, sqlx::Error> = sqlx::query_as("select id, username from users")
        .fetch_all(&data.pg)
        .await;

    match res {
        Ok(resx) => Ok(ApiResponse::JsonVec(resx)),
        Err(e) => Err(ApiError::InternalServerError(e.to_string()))
    }
}

async fn insert_user(State(data): State<Arc<AppState>>, Json(payload): Json<UserInsert> ) -> Result<ApiResponse<User>, ApiError>  {

    let get_username = sqlx::query_as::<_, User>("SELECT id, username, password FROM users where username = $1 LIMIT 1")
                    .bind(&payload.username)
                    .fetch_one(&data.pg)
                    .await;

    match payload.validate() {
        Ok(_) =>  {
            match get_username {
                Ok(x) => {
                    println!("{:?}", x);
        
                    let user_res = format!("username sudah ada dengan username : {}", x.username);
                    Err(ApiError::AlreadyExist(Json(user_res).to_string()))
                },
                Err(_) => {
                    let hashed = hash(&payload.password, 6);
        
                    match hashed {
                        Ok(x) =>{
                            let res = sqlx::query("insert into users (username, password) values ($1, $2)")
                                    .bind(&payload.username)
                                    .bind(x)
                                    .execute(&data.pg)
                                    .await;
                                
                            match res {
                                Ok(_) => Ok(ApiResponse::Created),
                                Err(e) => {
                                    Err(ApiError::InternalServerError(e.to_string()))
                                }
                            }
                        },
                        Err(y) => Err(ApiError::InternalServerError(y.to_string()))
                    }
                }
            }
        }
        Err(e) => Err(ApiError::BadRequest(e.to_string()))
    }

    
}

