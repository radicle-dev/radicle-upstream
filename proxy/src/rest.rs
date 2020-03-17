//! Defines the proxy's REST API.

/// Route handlers for the Org entity.
mod orgs;

/// Start the API server.
pub fn run() {
    let mut router = routes![];
    // add entity modules one by one
    router.extend(orgs::router());
    // mount the api endpoint
    rocket::ignite()
        .mount("/api/v1", router)
        // and if we ever need to make a new API version...
        // .mount("/api/v2", routes![])
        .launch();
}
