use librad::keys::SecretKey;
use librad::signer;

use coco::config;
use coco::{Lock, Peer};

pub async fn build_peer(
    tmp_dir: &tempfile::TempDir,
) -> Result<(Peer, Lock, signer::BoxedSigner), Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let signer = signer::BoxedSigner::from(key.clone());
    let store = kv::Store::new(kv::Config::new(tmp_dir.path().join("store")))?;

    let conf = config::default(key, tmp_dir.path())?;
    let (peer, state) = coco::into_peer_state(conf, signer.clone(), store).await?;

    Ok((peer, state, signer))
}
