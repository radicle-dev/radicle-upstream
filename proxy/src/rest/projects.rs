use librad::surf;
use radicle_registry_client::ed25519;
use rocket::http;
use rocket::request::Form;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::coco;
use crate::registry::Registry;
use crate::rest::lib::Ok;

#[get("/projects")]
/// TODO
fn get_projects() -> Json<Ok> {
    // TODO
    Json(Ok::new())
}

#[derive(Deserialize)]
struct ProjectCreationForm {
    path: String,
    publish: bool,
    name: String,
    description: String,
    default_branch: String,
    img_url: String,
}

#[derive(Serialize)]
struct ProjectJson {
    id: String,
    metadata: crate::project::Metadata,
}

#[post("/projects", format = "json", data = "<form>")]
/// TODO
fn create_project(
    librad_paths: State<librad::paths::Paths>,
    form: Json<ProjectCreationForm>,
) -> Json<ProjectJson> {
    // TODO parse body for project params
    if surf::git::git2::Repository::open(form.0.path.clone()).is_err() {
        coco::init_repo(form.0.path.clone()).expect("could not init repo");
    };

    let (id, meta) = coco::init_project(
        &librad_paths,
        &form.0.path,
        &form.0.name,
        &form.0.description,
        &form.0.default_branch,
        &form.0.img_url,
    )
    .expect("could not init project");

    let project = crate::project::Project {
        id: id.into(),
        metadata: meta.into(),
    };

    Json(ProjectJson {
        id: id.to_string(),
        metadata: project.metadata,
    })
}

#[get("/project/<project_id>")]
/// TODO
fn get_project(project_id: String, librad_paths: State<librad::paths::Paths>) -> Json<ProjectJson> {
    let meta = coco::get_project_meta(&librad_paths, &project_id)
        .expect("could not retrieve project meta");

    let id = librad::project::ProjectId::from_str(project_id.as_str()).expect("invalid project ID");
    let project = crate::project::Project {
        id,
        metadata: meta.into(),
    };

    Json(ProjectJson {
        id: project_id,
        metadata: project.metadata,
    })
}

#[derive(Deserialize)]
/// Form data structure
struct ProjectRegistrationForm {
    org_id: String,
    librad_id: String,
}

#[post("/project/<project_id>/register", format = "json", data = "<form>")]
/// Register a project by adding it to the Registry.
fn register_project(
    project_id: String,
    registry: State<Registry>,
    form: Json<ProjectRegistrationForm>,
) -> Json<Ok> {
    // TODO(xla): Get keypair from persistent storage.
    let fake_pair = ed25519::Pair::from_legacy_string("//Robot", None);
    let librad_id = librad::project::ProjectId::from_str(form.0.librad_id.as_str())
        .expect("could not convert librad ID to project ID");
    registry.register_project(&fake_pair, project_id, form.0.org_id, Some(librad_id));
    Json(Ok::new())
}

#[post("/project/<project_id>/unregister")]
/// TODO
fn unregister_project(project_id: String) -> http::Status {
    http::Status::new(501, "Not implemented.")
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
