// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

use std::{
    io::Write as _,
    process::{Command, Stdio},
};

use librad::profile::Profile;
use link_crypto::{BoxedSigner, PublicKey, SecretKey, SomeSigner};
use radicle_git_helpers::remote_helper;
use radicle_keystore::{crypto, pinentry::SecUtf8, FileStorage, Keystore};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let profile = Profile::load()?;
    let paths = profile.paths().to_owned();
    let key_file_path = paths.keys_dir().join("librad.key");

    let signer = if let Ok(signer) =
        lnk_clib::keys::ssh::signer(&profile, lnk_clib::keys::ssh::SshAuthSock::Env)
    {
        Some(signer)
    } else if std::env::var("RADICLE_UNSAFE_FAST_KEYSTORE") == Ok("1".to_string()) {
        Some(unsafe_fast_keystore_signer(key_file_path)?)
    } else {
        None
    };
    remote_helper::run(remote_helper::Config { signer })
}

fn unsafe_fast_keystore_signer(key_file_path: PathBuf) -> anyhow::Result<BoxedSigner> {
    let passphrase: SecUtf8 = if let Ok(value) = std::env::var("KEY_PASSPHRASE") {
        SecUtf8::from(value)
    } else {
        let mut child = Command::new("git")
            .args(&["credential", "fill"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.as_mut().expect("could not obtain stdin");
        stdin.write_all("url=rad://\nusername=radicle\n\n".to_string().as_bytes())?;

        let output = child.wait_with_output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let passphrase_: SecUtf8 = stdout
            .lines()
            .find_map(|line| line.strip_prefix("password=").map(SecUtf8::from))
            .ok_or_else(|| anyhow::anyhow!("couldn't obtain passphrase"))?;

        passphrase_
    };

    let keystore = FileStorage::<_, PublicKey, SecretKey, _>::new(
        &key_file_path,
        crypto::Pwhash::new(passphrase, *crypto::KDF_PARAMS_TEST),
    );
    let secret_key = keystore.get_key().map(|keypair| keypair.secret_key)?;
    Ok(SomeSigner { signer: secret_key }.into())
}
