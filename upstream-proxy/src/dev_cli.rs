// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use anyhow::Context as _;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
    #[clap(flatten)]
    opts: Opts,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Init(InitArgs),
    AddSshKey(AddSshKeyArgs),
}

#[derive(Debug, clap::Parser)]
struct Opts {
    #[clap(long, env)]
    /// Location of profile.
    lnk_home: std::path::PathBuf,
}

#[derive(Debug, clap::Parser)]
/// Initialize a Radicle and Upstream proxy profile with a key and local identity.
struct InitArgs {
    /// Local identity handle and key seed.
    handle: String,

    #[clap(long, default_value = "asdf")]
    /// Passphrase to encrypt the key with
    key_passphrase: String,
}

#[derive(Debug, clap::Parser)]
/// Add the Radicle key to the SSH agent
struct AddSshKeyArgs {
    #[clap(long, default_value = "asdf")]
    /// Passphrase to encrypt the key with
    key_passphrase: String,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();
    match args.command {
        Command::Init(init_args) => init(args.opts, init_args).await,
        Command::AddSshKey(add_key_args) => add_key(args.opts, add_key_args).await,
    }
}

async fn init(opts: Opts, args: InitArgs) -> anyhow::Result<()> {
    let profile = librad::profile::Profile::from_root(&opts.lnk_home, None)
        .context("failed to load profile")?;
    let key_path = profile.paths().keys_dir().join("librad.key");

    let keystore = crate::keystore::unsafe_fast_file(key_path);
    let secret_key = keystore
        .create_key_with_seed(
            secstr::SecUtf8::from(Vec::from(args.key_passphrase.clone())),
            Some(args.handle.as_ref()),
        )
        .context("failed to create key")?;

    let storage = librad::git::storage::Storage::open(profile.paths(), secret_key.clone())
        .context("failed to open librad storage")?;
    let person = lnk_identities::person::create::<()>(
        &storage,
        profile.paths().clone(),
        secret_key.clone().into(),
        link_identities::payload::Person {
            name: args.handle.into(),
        },
        vec![],
        vec![],
        lnk_identities::person::Creation::New { path: None },
    )
    .context("failed to create identity")?;

    let local_identity = lnk_identities::local::get(&storage, person.urn())
        .context("failed to get created person")?
        .ok_or_else(|| anyhow::anyhow!("person does not exist"))?;
    lnk_identities::local::set(&storage, local_identity).context("failed to set local identity")?;

    let peer_id = librad::PeerId::from(secret_key);

    let store_path = crate::config::store_dir(profile.id(), Some(opts.lnk_home.as_path()));

    let store = kv::Store::new(kv::Config::new(store_path).flush_every_ms(100))?;
    crate::session::initialize(&store, &[]).context("failed to initialize session")?;

    rad_common::git::configure_signing(profile.paths().git_dir(), &peer_id)
        .context("failed to configure monorepo for rad CLI")?;

    match add_key(
        opts,
        AddSshKeyArgs {
            key_passphrase: args.key_passphrase,
        },
    )
    .await
    {
        Ok(_) => {},
        Err(err) => eprintln!("{:?}", err.context("failed to add SSH key for identity")),
    };

    let output = serde_json::json!({
        "peerId": peer_id,
        "identityUrn": person.urn(),
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&output).expect("failed to serialize data")
    );

    Ok(())
}

async fn add_key(opts: Opts, args: AddSshKeyArgs) -> anyhow::Result<()> {
    use crate::keystore::Keystore as _;
    use radicle_keystore::sign::Signer as _;

    let profile = librad::profile::Profile::from_root(&opts.lnk_home, None)
        .context("failed to load profile")?;
    let key_path = profile.paths().keys_dir().join("librad.key");
    let keystore = crate::keystore::unsafe_fast_file(key_path);
    let key = match keystore.get(args.key_passphrase.into()) {
        Ok(key) => key,
        Err(err) => {
            if err.is_invalid_passphrase() {
                anyhow::bail!("invalid passphrase")
            } else {
                anyhow::bail!(err);
            }
        },
    };
    let agent = radicle_keystore::sign::SshAgent::new(key.public_key());
    radicle_keystore::sign::ssh::add_key::<tokio::net::UnixStream>(&agent, key.into(), &[])
        .await
        .context("failed to add key to agent")?;
    Ok(())
}
