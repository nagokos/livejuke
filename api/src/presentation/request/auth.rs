use serde::Deserialize;
use utoipa::ToSchema;

use crate::domain::{
    authentication::{email::Email, model::EmailCredentials},
    user::model::NewUser,
};

#[derive(Deserialize, Clone, ToSchema)]
pub struct RegisterEmailInput {
    display_name: String,
    #[schema(value_type = String)]
    email: Email,
    password: String,
}

impl RegisterEmailInput {
    pub fn into_parts(self) -> (NewUser, EmailCredentials) {
        (
            NewUser {
                display_name: self.display_name,
            },
            EmailCredentials {
                email: self.email,
                password: self.password,
            },
        )
    }
}

#[derive(Deserialize, ToSchema)]
pub struct LoginEmailInput {
    #[schema(value_type = String)]
    email: Email,
    password: String,
}

impl From<LoginEmailInput> for EmailCredentials {
    fn from(value: LoginEmailInput) -> Self {
        Self {
            email: value.email,
            password: value.password,
        }
    }
}
