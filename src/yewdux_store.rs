use yewdux::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct AuthStore {
    pub loading: bool,
    pub is_authenticated: bool,
}

pub type AuthStoreType = BasicStore<AuthStore>;
