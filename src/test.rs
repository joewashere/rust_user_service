use rocket::Rocket;
use rocket::State;
use rocket::tokio;
use sqlx::sqlite::SqlitePool;

#[macro_use] extern crate rocket;

#[get("/")]
async fn index(pool: &State<SqlitePool>) -> &'static str {
    let conn = pool.acquire().await;
    match conn {
        Ok(_) => "Database connection successful!",
        Err(_) => "Failed to connect to the database.",
    }
}

#[launch]
async fn rocket() -> _ {
    let pool = SqlitePool::connect("sqlite:users.db").await.expect("Failed to create database pool.");
    rocket::build()
        .manage(pool)
        .mount("/", routes![index])
}

// The main function to run the Rocket application
fn main() {
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(rocket()).launch();
}
