use serde::{Serialize};

#[derive(Serialize)]
/// standard 200 { ok: true } reply
pub struct Ok {
    ok: bool,
}

impl Ok {
    pub fn new () -> Ok {
        Ok { ok: true }
    }
}
