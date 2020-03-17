#[get("/orgs")]
/// TODO
fn get_orgs() -> &'static str {
    "org 1, org 2, org 3"
}

#[post("/orgs")]
/// TODO
fn create_org() -> &'static str {
    // TODO parse body for org params
    "new org"
}

#[get("/org/<org_id>")]
/// TODO
fn get_org(org_id: String) -> String {
    format!("org {}", org_id)
}

#[post("/org/<org_id>/register")]
/// TODO
fn register_org(org_id: String) -> String {
    format!("org {} registered", org_id)
}

#[post("/org/<org_id>/unregister")]
/// TODO
fn unregister_org(org_id: String) -> String {
    format!("org {} unregistered", org_id)
}

/// TODO
pub fn router() -> Vec<rocket::Route> {
    routes![create_org, get_org, get_orgs, register_org, unregister_org]
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn hello_world() {
        let rocket = rocket::ignite().mount("/", crate::rest::orgs::router());
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/orgs").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("org 1, org 2, org 3".into()));
    }
}
