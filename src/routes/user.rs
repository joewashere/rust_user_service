use rocket::http::Status;
use rocket::State;
use rocket::serde::json::Json;
use rocket::request::{self, FromRequest, Request, Outcome};
use serde_json::{json, Value};
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx;
use serde::{Serialize, Deserialize};
use config::Config;
use crate::auth::jwt::{generate_token, validate_token};
use crate::db::conn::UsersDbConn;

#[derive(Debug)]
pub struct BearerToken(pub String);

#[derive(FromForm, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
}

#[derive(FromForm, Serialize, Deserialize)]
pub struct UserDelete {
    username: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,              // Subject (whom the token refers to, typically user ID)
    pub exp: usize,               // Expiration time
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
    type Error = std::io::Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((rocket::http::Status::BadRequest, std::io::Error::new(std::io::ErrorKind::Other, "Expected only one Authorization header")));
        }

        let key = keys[0];
        if key.starts_with("Bearer ") {
            Outcome::Success(BearerToken(key[7..].to_string()))
        } else {
            Outcome::Failure((rocket::http::Status::BadRequest, std::io::Error::new(std::io::ErrorKind::Other, "Invalid Authorization header format")))
        }
    }
}

#[post("/v1/register", data = "<user>")]
pub async fn register_v1(pool: &State<UsersDbConn>, user: Json<User>) -> Result<Status, rocket::response::Debug<sqlx::Error>> {
    // Get a connection from the pool
    let mut conn = match pool.0.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to acquire database connection: {:?}", e);
            return Err(rocket::response::Debug(sqlx::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Database error"))));
        }
    };

    let hashed_password = hash(&user.password, DEFAULT_COST).expect("Hashing error");
    sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)")
        .bind(&user.username)
        .bind(&hashed_password)
        .execute(&mut conn)
        .await?;

    Ok(Status::Ok)
}

#[post("/v1/login", data = "<user>")]
pub async fn login_v1(settings: &State<Config>, pool: &State<UsersDbConn>, user: Json<User>) -> Result<(Status, Json<Value>), rocket::response::Debug<sqlx::Error>> {
    let mut conn = match pool.0.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to acquire database connection: {:?}", e);
            return Err(rocket::response::Debug(sqlx::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Database error"))));
        }
    };

    let row: Result<(String,), _> = sqlx::query_as("SELECT password FROM users WHERE username = ?")
        .bind(&user.username)
        .fetch_one(&mut conn)
        .await;

    match row {
        Ok((hashed_password,)) => {
            match verify(&user.password, &hashed_password) {
                Ok(valid) => {
                    if valid {
                        match generate_token(settings.inner(), &user.username) {
                            Ok(token) => Ok((Status::Ok, Json(json!({
                                "status": "success",
                                "token": token
                            })))),
                            Err(_) => {
                                // Handle the JWT error here.
                                Ok((Status::Unauthorized, Json(json!({
                                    "status": "error",
                                    "reason": "Token generation error"
                                }))))
                            }
                        }
                    } else {
                        Ok((Status::Unauthorized, Json(json!({
                            "status": "error",
                            "reason": "Invalid password"
                        }))))
                    }
                },
                Err(_) => {
                    // Handle the bcrypt error here.
                    Ok((Status::Unauthorized, Json(json!({
                        "status": "error",
                        "reason": "Password verification error"
                    }))))
                }
            }
        },
        _ => Ok((Status::Unauthorized, Json(json!({
            "status": "error",
            "reason": "User not found"
        })))),
    }
}

#[delete("/v1/delete", data = "<user>")]
pub async fn delete_v1(settings: &State<Config>, pool: &State<UsersDbConn>, user: Json<UserDelete>, token: BearerToken) -> Result<Status, rocket::response::Debug<sqlx::Error>> {
    match validate_token(&settings, &token) {
        Ok(claims) => {
            if claims.sub == user.username {
                let mut conn = match pool.0.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to acquire database connection: {:?}", e);
            return Err(rocket::response::Debug(sqlx::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Database error"))));
        }
    };
                sqlx::query("DELETE FROM users WHERE username = ?")
                    .bind(&user.username)
                    .execute(&mut *conn)
                    .await?;
                Ok(Status::Ok)
            } else {
                Ok(Status::Unauthorized)
            }
        },
        Err(_) => Ok(Status::Unauthorized)
    }
}