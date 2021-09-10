#[cfg(test)]
use super::*;
use rocket::http::Status;

impl Test {
    fn dummy() -> Test {
        Test {
            title: "Title".to_string(),
            body: "This is the body".to_string()
        }
    }
}

mod test_helper {
    use crate::rocket;
    use rocket::local::blocking::Client;

    pub fn client() -> Client {
        Client::tracked(rocket()).expect("valid rocket instance")
    }
}


#[test]
fn test_index() {
    let client = test_helper::client();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_index_json() {
    let msg = Test::dummy();
    let client = test_helper::client();
    let response = client.get("/").dispatch();
    assert_eq!(response.into_json(), Some(msg));
}
