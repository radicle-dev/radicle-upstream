use std::convert::TryInto as _;
use std::env;
use std::ffi;
use std::path::Path;

use proxy::coco;
use proxy::error;
use proxy::keystore;
use proxy::project;

#[tokio::test]
async fn can_checkout() -> Result<(), error::Error> {
    let tmp_dir = tempfile::tempdir().expect("failed to create temdir");

    env::set_var("RAD_HOME", tmp_dir.path());
    let paths = coco::config::Paths::FromRoot(tmp_dir.path().to_path_buf()).try_into()?;
    let mut keystore =
        keystore::Keystorage::new(&paths, keystore::SecUtf8::from("radicle-upstream"));
    let key = keystore.init_librad_key()?;
    let config = coco::config::configure(paths, key.clone(), vec![]);
    let api = coco::Api::new(config).await?;

    let handle = "cloudhead";
    let owner = api.init_owner(&key, handle)?;

    let platinum_project = coco::control::replicate_platinum(
        &api,
        &key,
        &owner,
        "git-platinum",
        "fixture data",
        "master",
    )?;

    let path = tmp_dir.path().join("projects").join("git-platinum");
    let exe_path = exe_path()?;
    project::Checkout::new(platinum_project, path, exe_path).run()?;

    Ok(())
}

/// Set up the PATH env variable used for running the checkout.
fn exe_path() -> Result<ffi::OsString, error::Error> {
    let exe_path = env!("CARGO_BIN_EXE_git-remote-rad");
    let exe_path = Path::new(exe_path.strip_suffix("git-remote-rad").unwrap());

    let paths = env::var_os("PATH").map_or(vec![exe_path.to_path_buf()], |path| {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        paths.push(exe_path.to_path_buf());
        paths.reverse();
        paths
    });

    Ok(env::join_paths(paths)?)
}
