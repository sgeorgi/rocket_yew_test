#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::{Serialize};

#[derive(Debug, Serialize)]
struct Test {
    title: String,
    body: String
}

#[get("/")]
fn index() -> Json<Test> {
    let msg = Test {
        title: "Title".to_string(),
        body: "This is the body".to_string()
    };
    Json(msg)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
