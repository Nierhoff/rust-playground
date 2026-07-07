use rocket::local::blocking::Client;

#[test]
fn hello() {
    let client = Client::tracked(super::rocker_build()).unwrap();
    let response = client.get("/hello").dispatch();
    assert_eq!(response.into_string(), Some("Hello, OpenAPI!".into()));
}
