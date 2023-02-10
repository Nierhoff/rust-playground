use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "snake_case")]
pub struct Login {
    #[validate(min_length = 5)]
    #[validate(max_length = 20)]
    pub username: String,
    #[validate(min_length = 8)]
    #[validate(max_length = 8)]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "snake_case")]
pub struct User {
    #[validate(min_length = 5)]
    #[validate(max_length = 20)]
    pub email: String,
    #[validate(min_length = 5)]
    #[validate(max_length = 20)]
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "snake_case")]
pub struct UserSignUp {
    #[validate(min_length = 5)]
    #[validate(max_length = 20)]
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

    use crate::{Login, User};

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
    fn it_login_ok() {
        let u = Login {
            username: "johndoe".to_string(),
            password: "Abc12345".to_string(),
        };
        assert!(u.validate().is_ok());
    }
}
