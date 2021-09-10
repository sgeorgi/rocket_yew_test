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
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use crate::Test;

    #[test]
    fn test_index() {
        let msg = Test {
            title: "Title".to_string(),
            body: "This is the body".to_string()
        };

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_json(), Some(msg));
    }
}
