use radicle_registry_client::ed25519;
use rocket::http;
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

// #[post("/projects", format = "json", data = "<form>")]
// /// TODO
// fn create_project(
//     librad_paths: State<Paths>,
//     form: Json<ProjectRegistrationForm>,
// ) -> Json<ProjectJson> {
//     // TODO parse body for project params
//     if surf::git::git2::Repository::open(form.path.clone()).is_err() {
//         coco::init_repo(form.path.clone())?;
//     };
//
//     let (id, meta) = coco::init_project(
//         librad_paths.read().expect("unable to acquire read lock"),
//         &form.path,
//         &form.name,
//         &form.description,
//         &form.default_branch,
//         &form.img_url,
//     )?;
//
//     Json(ProjectJson { project: project::Project::Git {
//         id: librad::project::ProjectId::from(id),
//         metadata: meta.into(),
//     }})
// }

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
    // TODO(garbados): convert string to `librad::project::ProjectId`.
    let librad_id = form.librad_id.map(|id| project::ProjectId::from_str(&id));
    registry.register_project(&fake_pair, project_id, form.org_id, librad_id);
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
