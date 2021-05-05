use radicle_daemon::remote_helper;

fn main() -> anyhow::Result<()> {
    remote_helper::run(Default::default())
}
