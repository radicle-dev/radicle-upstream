//! Infrastructure to provide `OpenAPI` documentation for the exposed endpoints.

use warp::{document, path, reply, Filter, Rejection, Reply};

/// GET /
/// GET /index.html
pub fn index_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path::end().or(path("index.html")).map(|_| {
        warp::http::Response::builder()
            .header("content-type", "text/html; charset=utf-8")
            .body(INDEX_HTML)
    })
}

/// GET /opnapi.json
pub fn describe_filter<F: Filter>(
    routes: &F,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let docs = document::to_openapi(document::describe(routes));

    path!("openapi.json").map(move || reply::json(&docs))
}

/// Static HTML served for the docs endpoint.
const INDEX_HTML: &str = r#"
<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <script type="module" src="https://unpkg.com/rapidoc/dist/rapidoc-min.js"></script>
</head>
<body>
  <rapi-doc 
    spec-url = "/docs/openapi.json"
    render-style = "read"
    default-schema-tab = "model"
    schema-style = "table"
    sort-tags = true
    sort-endpoints-by = "path"
    schema-description-expanded = false
    show-info = false
    show-header = false
    allow-authentication = false
    allow-server-selection = false

    nav-bg-color = '#3e4b54'
    nav-accent-color = '#fd6964'
    primary-color = '#ea526f'
    >
  </rapi-doc>
</body> 
</html>
"#;
