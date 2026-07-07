use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use schemars::JsonSchema;

pub mod types;

pub mod routes {
    pub const LOGIN: &str = "/api/login";
    pub const USER: &str = "/user";
    pub const USERS: &str = "/users";
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    #[validate(min_length = 5)]
    #[validate(max_length = 20)]
    pub username: String,
    #[validate(min_length = 8)]
    #[validate(max_length = 8)]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[validate(min_length = 5)]
    #[validate(max_length = 20)]
    #[validate(pattern = r"^\S+\@\S+\.\S+$")]
    pub email: String,
    #[validate(min_length = 5)]
    #[validate(max_length = 20)]
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSignUp {
    #[validate(min_length = 5)]
    #[validate(max_length = 20)]
    #[validate(pattern = r"^\S+\@\S+\.\S+$")]
    pub email: String,
    #[validate(min_length = 5)]
    #[validate(max_length = 20)]
    pub username: String,
    #[validate(min_length = 8)]
    #[validate(max_length = 8)]
    pub password: String,
}

#[cfg(test)]
mod tests {
    use serde_valid::Validate;

    use crate::{Login, User, UserSignUp};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_user_ok() {
        let u = User {
            username: "johndoe".to_string(),
            email: "john@doe.com".to_string(),
        };
        assert!(u.validate().is_ok());
    }

    #[test]
    fn it_user_not_ok() {
        let u = User {
            username: "johndoe".to_string(),
            email: "john@doe.".to_string(),
        };
        assert!(u.validate().is_err());
    }

    #[test]
    fn it_login_ok() {
        let u = Login {
            username: "johndoe".to_string(),
            password: "Abc12345".to_string(),
        };
        assert!(u.validate().is_ok());
    }

    #[test]
    fn it_signup_not_ok() {
        let u = UserSignUp {
            email: "".to_string(),
            username: "johndoe".to_string(),
            password: "Abc12345".to_string(),
        };
        assert!(u.validate().is_err());
    }

    #[test]
    fn it_signup_ok() {
        let u = UserSignUp {
            email: "a@b.c".to_string(),
            username: "johndoe".to_string(),
            password: "Abc12345".to_string(),
        };
        assert!(u.validate().is_ok());
    }
}
