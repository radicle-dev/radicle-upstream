use librad::paths::Paths;
use librad::project;
use librad::surf;
use librad::surf::git::git2;
use radicle_registry_client::ed25519;
use rocket::http;
use rocket::response;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

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
fn get_project(project_id: String) -> String {
    let meta = coco::get_project_meta(
        &ctx.librad_paths
            .read()
            .expect("unable to acquire read lock"),
        &id.to_string(),
    ).expect("could not retrieve project meta");


    Json(NewProject({ project: project::Project {
        id: librad::project::ProjectId::from(id),
        metadata: meta.into(),
    }}))
}

#[derive(Deserialize)]
/// Form data structure
struct ProjectRegistrationForm {
    org_id: String,
    librad_id: String
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
    let librad_id = form.librad_id.map(|id| {
        project::ProjectId::from_str(&id)
    });
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
