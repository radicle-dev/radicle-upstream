//! Defines the proxy's REST API.

/// Route handlers for the Org entity.
mod orgs;
mod projects;

use crate::registry::Registry;

/// Start the API server.
pub fn run(
    librad_paths: librad::paths::Paths,
    registry_client: radicle_registry_client::Client,
) {
    let client = Registry::new(registry_client);
    let mut router = routes![];
    // add entity modules one by one
    router.extend(orgs::router());
    router.extend(projects::router());
    // mount the api endpoint
    rocket::ignite()
        .manage(librad_paths)
        .manage(client)
        .mount("/api/v1", router)
        // and if we ever need to make a new API version...
        // .mount("/api/v2", routes![])
        .launch();
}
