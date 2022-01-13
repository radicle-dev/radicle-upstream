// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use anyhow::Context as _;

#[derive(Debug, clap::Parser)]
/// Initialize a Radicle Proxy profile with a key and local identity.
struct Args {
    /// Local identity handle and key seed.
    handle: String,

    #[clap(long, env)]
    /// Location of profile.
    rad_home: std::path::PathBuf,

    #[clap(long, default_value = "asdf")]
    /// Passphrase to encrypt the key with
    key_passphrase: String,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();

    let profile = librad::profile::Profile::from_root(&args.rad_home, None)
        .context("failed to load profile")?;
    let key_path = profile.paths().keys_dir().join("librad.key");

    let keystore = crate::keystore::unsafe_fast_file(key_path);
    let secret_key = keystore
        .create_key_with_seed(
            secstr::SecUtf8::from(Vec::from(args.key_passphrase)),
            Some(args.handle.as_ref()),
        )
        .context("failed to create key")?;

    let storage = librad::git::storage::Storage::open(profile.paths(), secret_key.clone())
        .context("failed to open librad storage")?;
    let person = rad_identities::person::create::<()>(
        &storage,
        profile.paths().clone(),
        secret_key.clone().into(),
        link_identities::payload::Person {
            name: args.handle.into(),
        },
        vec![],
        vec![],
        rad_identities::person::Creation::New { path: None },
    )
    .context("failed to create identity")?;

    let local_identity = rad_identities::local::get(&storage, person.urn())
        .context("failed to get created person")?
        .ok_or_else(|| anyhow::anyhow!("person does not exist"))?;
    rad_identities::local::set(&storage, local_identity).context("failed to set local identity")?;

    let peer_id = librad::PeerId::from(secret_key);
    let identity = crate::identity::Identity::from((peer_id, person));
    let identity_urn = identity.urn.clone();

    let store_path = crate::config::store_dir(profile.id(), Some(args.rad_home.as_path()));

    let store = kv::Store::new(kv::Config::new(store_path).flush_every_ms(100))?;
    crate::session::initialize(&store, identity, &[]).context("failed to initialize session")?;

    let output = serde_json::json!({
        "peerId": peer_id,
        "identityUrn": identity_urn,
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&output).expect("failed to serialize data")
    );

    Ok(())
}
