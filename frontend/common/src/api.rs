pub mod model {
    use serde::{Deserialize, Serialize};
    use serde_valid::Validate;

    #[derive(Validate, Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Login {
        #[validate(pattern = r"^\d{5}$")]
        pub email: String,
        #[validate(pattern = r"^.{8,}$")]
        pub password: String,
    }

    #[derive(Validate, Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LoginResponse {
        pub ok: bool,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
}

pub mod routes {
    pub const LOGIN: &str = "/api/login";
    pub const USER: &str = "/user";
    pub const USERS: &str = "/users";
}
