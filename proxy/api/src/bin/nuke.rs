use std::{fs::remove_dir_all, io::ErrorKind, process::exit};

use log::{info, trace};

use coco::control;

use api::{config, env};

fn main() {
    env::set_if_unset("RUST_BACKTRACE", "full");
    env::set_if_unset("RUST_LOG", "info");
    pretty_env_logger::init();

    info!("Nuking application state...");
    match remove_dir_all(config::dirs().data_dir()) {
        Ok(_) => info!("done"),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                info!("already gone");
            } else {
                trace!("{:?}", err);
                exit(1);
            }
        },
    };

    info!("Nuking coco state...");
    match control::nuke_monorepo() {
        Ok(_) => info!("done"),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                info!("already gone");
            } else {
                trace!("{:?}", err);
                exit(1);
            }
        },
    };
}
