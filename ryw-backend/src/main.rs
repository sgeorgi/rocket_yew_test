#[macro_use]
extern crate rocket;

mod tests;

use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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


