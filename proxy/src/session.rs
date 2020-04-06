use crate::identity;

pub struct Session {
    pub identity: Option<identity::Identity>,
}
