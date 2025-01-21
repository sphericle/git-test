use crate::get::get_all::get_all;
use rocket::{self, serde::json::json};
mod get;
mod types;
mod test;
use sqlx::{sqlite::SqlitePool, types::JsonValue};

#[rocket::get("/insert")]
async fn add_all(pool: &rocket::State<SqlitePool>) -> rocket::serde::json::Json<JsonValue> {
    let list: Vec<types::Level> = get_all();
    
    for level in list {
        let pool = pool.inner();
        let row = sqlx::query("INSERT INTO levels (level_id, name, verifier, verification, percent_to_qualify, song_name, song_link, difficulty) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(level.id)
            .bind(level.name)
            .bind(level.verifier)
            .bind(level.verification)
            .bind(level.percent_to_qualify)
            .bind(level.song_name)
            .bind(level.song_link)
            .bind(level.difficulty.as_int())
            .execute(pool)
            .await
            .expect("failed to insert level into db");

        for record in level.records {
            sqlx::query("INSERT INTO records (level_id, user, link, mobile, enjoyment, hz) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(row.last_insert_rowid())
                .bind(record.user)
                .bind(record.link)
                .bind(if record.mobile { 1 } else { 0 })
                .bind(record.enjoyment.unwrap_or_default())
                .bind(record.hz)
                .execute(pool)
                .await
                .expect("failed to insert record into db");
        }
    }
    rocket::serde::json::Json(json!({ "status": "success" }))
}

#[rocket::get("/list")]
fn list_all() -> rocket::serde::json::Json<JsonValue> {
    let list: Vec<types::Level> = get_all();

    
    
    rocket::serde::json::Json(serde_json::json!(list))
}

#[rocket::main]
async fn main() {
    // setup sqlite pool
    let pool = SqlitePool::connect("sqlite:./db.db")
        .await
        .expect("failed to connect to sqlite");

    rocket::build()
        .mount("/", rocket::routes![add_all, list_all])
        .manage(pool)
        .launch()
        .await
        .expect("server failed to launch");
}
