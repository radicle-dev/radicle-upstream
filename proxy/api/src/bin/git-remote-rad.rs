use coco::remote_helper;

fn main() -> anyhow::Result<()> {
    remote_helper::run(remote_helper::Config::default())
}
