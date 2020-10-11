use std::{fs::File, io::prelude::*};

use librad::{keys::SecretKey, peer::PeerId};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = SecretKey::new();
    let peer_id = PeerId::from(key);

    let mut file = File::create("/tmp/seed.key")?;
    file.write_all(key.as_ref())?;

    println!("{}", peer_id);

    Ok(())
}
