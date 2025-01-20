use crate::get::get_all::get_all;
mod get;
mod types;
mod test;
use rocket;

#[rocket::get("/")]
fn return_all() -> rocket::serde::json::Json<Vec<types::Level>> {
    let list: Vec<types::Level> = get_all();
    rocket::serde::json::Json(list)
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", rocket::routes![return_all])
        .launch()
        .await
        .expect("server failed to launch");
}
