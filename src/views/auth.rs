use serde::{Deserialize, Serialize};

use crate::models::{_entities::users, users::users::Role};

#[derive(Debug, Deserialize, Serialize, ts_rs::TS)]
#[ts(export)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
    pub email: String,
    pub role: Role,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, token: &str) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: user.email_verified_at.is_some(),
            email: user.email.clone(),
            role: user.role.clone(),
        }
    }
}
