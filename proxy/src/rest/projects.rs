use radicle_registry_client::ed25519;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::registry::Registry;

#[derive(Serialize)]
/// standard 200 { ok: true } reply
struct Ok {
    ok: bool,
}

#[get("/projects")]
/// TODO
fn get_projects() -> &'static str {
    "project 1, project 2, project 3"
}

#[post("/projects")]
/// TODO
fn create_project() -> &'static str {
    // TODO parse body for project params
    "new project"
}

#[get("/project/<project_id>")]
/// TODO
fn get_project(project_id: String) -> String {
    format!("project {}", project_id)
}

#[derive(Deserialize)]
/// Form data structure
struct ProjectRegistrationForm {
    org_id: String,
    librad_id: Option<String>,
}

#[post("/project/<project_id>/register", format = "json", data = "<form>")]
/// TODO
fn register_project(
    project_id: String,
    registry: State<Registry>,
    form: Json<ProjectRegistrationForm>,
) -> Json<Ok> {
    // TODO(xla): Get keypair from persistent storage.
    let fake_pair = ed25519::Pair::from_legacy_string("//Robot", None);
    // TODO(garbados): convert string to `librad::project::ProjectId`.
    let librad_id = form
        .librad_id
        .map(|id| librad::project::ProjectId::from_str(&id.to_string()));
    registry.register_project(&fake_pair, project_id, form.org_id, librad_id);
    Json(Ok { ok: true })
}

#[post("/project/<project_id>/unregister")]
/// TODO
fn unregister_project(project_id: String) -> String {
    format!("project {} unregistered", project_id)
}

/// TODO
pub fn router() -> Vec<rocket::Route> {
    routes![
        create_project,
        get_project,
        get_projects,
        register_project,
        unregister_project
    ]
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn hello_world() {
        let rocket = rocket::ignite().mount("/", crate::rest::projects::router());
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/projects").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some("project 1, project 2, project 3".into())
        );
    }
}
