use rocket::http;
use rocket::response;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[get("/project/<project_id>/branches")]
/// TODO
fn get_project_branches (
    // TODO
) -> &'static str {
    // TODO
}

#[get("/project/<project_id>/tags")]
/// TODO
fn get_project_tags (
    // TODO
) -> &'static str {
    // TODO
}

#[get("/project/<project_id>/tree/<revspec>")]
/// TODO
fn get_project_tree (
    // TODO
) -> &'static str {
    // TODO
}

#[get("/project/<project_id>/trees")]
/// TODO
fn get_project_trees (
    // TODO
) -> &'static str {
    // TODO
}

/// TODO
pub fn router() -> Vec<rocket::Route> {
    routes![
        get_project_branches,
        get_project_tags,
        get_project_tree,
        get_project_trees
    ]
}
