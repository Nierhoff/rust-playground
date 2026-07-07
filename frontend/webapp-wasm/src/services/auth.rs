use super::{request_get, request_post, request_put};
use crate::error::Error;
use model::types::*;

/// Get current user info
pub async fn current() -> Result<UserInfo, Error> {
    request_get::<UserInfo>(common::api::routes::USER.to_string()).await
}

/// Login a user
pub async fn login(login_info: LoginInfo) -> Result<UserInfo, Error> {
    request_post::<LoginInfo, UserInfo>( common::api::routes::LOGIN.to_string(), login_info).await
}

/// Register a new user
pub async fn register(register_info: RegisterInfo) -> Result<UserInfo, Error> {
    request_post::<RegisterInfo, UserInfo>(common::api::routes::USERS.to_string(), register_info).await
}

pub async fn save(user_update_info: UserUpdateInfo) -> Result<UserInfo, Error> {
    request_put::<UserUpdateInfo, UserInfo>(common::api::routes::USER.to_string(), user_update_info)
        .await
}
