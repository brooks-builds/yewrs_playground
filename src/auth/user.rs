use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub name: String,
}
