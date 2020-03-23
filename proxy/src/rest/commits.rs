use rocket::http;
use rocket::response;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[get("/project/<project_id>/commits")]
/// TODO
fn get_project_commits (
    // TODO
) -> &'static str {
    // TODO
}

#[get("/project/<project_id>/commit/<hash>")]
/// TODO
fn get_project_commit (
    // TODO
) -> &'static str {
    // TODO
}

#[get("/project/<project_id>/commit/<hash>/blob/<path>")]
/// TODO
fn get_project_commit_blob (
    // TODO
) -> &'static str {
    // TODO
}

/// TODO
pub fn router() -> Vec<rocket::Route> {
    routes![
        get_project_commit_blob,
        get_project_commit,
        get_project_commits,
    ]
}
