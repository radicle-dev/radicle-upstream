use radicle_daemon::{
    keys::{PublicKey, SecretKey},
    profile::Profile,
    signer,
};
use radicle_git_helpers::remote_helper;
use radicle_keystore::{crypto, pinentry::SecUtf8, FileStorage, Keystore};
use std::{
    io::Write as _,
    process::{Command, Stdio},
};

const SECRET_KEY_FILE: &str = "librad.key";

fn main() -> anyhow::Result<()> {
    let signer = if std::env::var("RADICLE_UNSAFE_FAST_KEYSTORE") == Ok("1".to_string()) {
        Some(unsafe_fast_keystore_signer()?)
    } else {
        None
    };
    remote_helper::run(remote_helper::Config { signer })
}

fn unsafe_fast_keystore_signer() -> anyhow::Result<signer::BoxedSigner> {
    let profile = Profile::load()?;
    let paths = profile.paths().to_owned();
    let file = paths.keys_dir().join(SECRET_KEY_FILE);
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
    let passphrase: SecUtf8 = stdout
        .lines()
        .find_map(|line| line.strip_prefix("password=").map(SecUtf8::from))
        .ok_or_else(|| anyhow::anyhow!("couldn't obtain passphrase"))?;

    let keystore = FileStorage::<_, PublicKey, SecretKey, _>::new(
        &file,
        crypto::Pwhash::new(passphrase, *crypto::KDF_PARAMS_TEST),
    );
    let secret_key = keystore.get_key().map(|keypair| keypair.secret_key)?;
    Ok(signer::SomeSigner { signer: secret_key }.into())
}
