// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use anyhow::Context as _;

/// Provides the following endpoints:
///
/// * `POST /identities` to create local identity
/// * `PUT /identities` to update local identity metadata
/// * `GET /identities/remote/:urn` to get information about a replicated identity
pub fn router() -> axum::Router {
    axum::Router::new()
        .route(
            "/identities",
            axum::routing::post(create_local).put(update_local),
        )
        .route("/identities/remote/:urn", axum::routing::get(get_person))
}

async fn create_local(
    super::extract::UnsealedContext(ctx): super::extract::UnsealedContext,
    metadata: axum::extract::Json<crate::identity::Metadata>,
) -> Result<impl axum::response::IntoResponse, super::Error> {
    let current_session =
        crate::session::get_current(&ctx.rest.store).context("failed to get current session")?;
    if current_session.is_some() {
        return Err(super::Error::Custom {
            status_code: http::StatusCode::BAD_REQUEST,
            variant: "SESSION_IN_USE",
            message: "A session already exists".to_string(),
            details: None,
        });
    }

    let id = crate::identity::create(ctx.peer.librad_peer(), metadata.0)
        .await
        .context("failed to create identity")?;

    crate::session::initialize(&ctx.rest.store, &[]).context("failed to initialize context")?;

    Ok((http::StatusCode::CREATED, axum::response::Json(id)))
}

async fn update_local(
    super::extract::UnsealedContext(ctx): super::extract::UnsealedContext,
    metadata: axum::extract::Json<crate::identity::Metadata>,
) -> Result<impl axum::response::IntoResponse, super::Error> {
    let id = crate::identity::update(ctx.peer.librad_peer(), metadata.0)
        .await
        .context("failed to update identity")?;

    Ok((http::StatusCode::CREATED, axum::response::Json(id)))
}

async fn get_person(
    super::extract::UnsealedContext(ctx): super::extract::UnsealedContext,
    path: super::extract::Path<super::extract::Urn>,
) -> Result<axum::response::Json<crate::identity::Person>, super::Error> {
    let urn = path.0 .0;
    let maybe_person_identity = ctx
        .peer
        .librad_peer()
        .using_storage({
            let urn = urn.clone();
            move |storage| lnk_identities::person::get(&storage, &urn)
        })
        .await
        .context("failed to get librad storage")?
        .context(format!("failed to get person {urn}"))?;

    match maybe_person_identity {
        Some(person_identity) => {
            let person = crate::identity::Person::from(person_identity);
            Ok(axum::response::Json(person))
        },
        None => Err(super::Error::Custom {
            status_code: http::StatusCode::NOT_FOUND,
            variant: "NOT_FOUND",
            message: "Person not found".to_string(),
            details: None,
        }),
    }
}
