use self::auth0::Auth0;

pub mod auth0;
pub mod user;

#[derive(Clone, PartialEq, Default)]
pub struct AppContext {
    pub auth0: Auth0,
}
