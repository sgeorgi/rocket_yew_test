#[macro_use]
extern crate rocket;

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

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_index_json() {
        let msg =  Test {
            title: "Title".to_string(),
            body: "This is the body".to_string()
        };
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.into_json(), Some(msg));
    }
}
