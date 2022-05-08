use yewdux::prelude::*;

use crate::auth::auth0::Auth0;

pub type AuthStoreType = BasicStore<Auth0>;
