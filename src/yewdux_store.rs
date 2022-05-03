#[derive(Debug, Clone)]
pub struct AuthStore {
    pub loading: bool,
    pub is_authenticated: bool,
}

impl Default for AuthStore {
    fn default() -> Self {
        Self {
            loading: true,
            is_authenticated: false,
        }
    }
}
