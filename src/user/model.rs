use serde::{
    Serialize,
    Deserialize
};
use sqlx::FromRow;
use validator::{
    Validate, 
    ValidationError
};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[0-9a-zA-Z]{2,}$").unwrap();
    static ref RE_PASSWORD: Regex = Regex::new(r"^.*?[!@#$%^&*()].*$").unwrap();
}

#[derive(Serialize, FromRow)]
pub struct User {
    pub id_user: i32,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct In<T> {
    pub user: T
}

#[derive(Deserialize, Validate)]
pub struct UserPayload {
    #[serde(
        deserialize_with = "capitalize_option_words"
    )]
    pub full_name: Option<String>,

    #[serde(
        deserialize_with = "to_option_lowercase"
    )]
    #[validate(
        email(
            message = "invalid email format."
        ),
        custom(
            function = "validate_email_domain",
            message = "invalid email format or domain. supported domains: gmail.com, icloud.com, yahoo.com, outlook.com"
        )
    )]
    pub email: Option<String>,

    #[validate(
        length(
            min = 9,
            max = 14,
            message = "phone number must be between 9 to 14."
        ),
        custom(
            function = "validate_phone_number",
            message = "invalid phone number format. must contain only digits."
        )
    )]
    pub phone_number: Option<String>,

    #[serde(
        deserialize_with = "to_lowercase"
    )]
    #[validate(
        length(
            min = 2,
            max = 25,
            message = "username length must be between 2 to 25 characters."
        ),
        regex(
            path = "RE_USERNAME",
            message = "username must consist of alphanumeric characters and be at least 2 characters long."
        )
    )]
    pub username: String,

    #[validate(
        length(
            min = 8,
            message = "password length must be at least 8 characters."
        ),
        custom(
            function = "validate_password",
            message = "password must contain at least one upper case, lower case, number and 8 characters long. don't use spaces."
        ),
        regex(
            path = "RE_PASSWORD",
            message = "pasword must be at least contain one special character."
        )
    )]
    pub password: String,

    #[validate(
        must_match(
            other = "password",
            message = "password do not match. password and confirm_password must be the same."
        )
    )]
    pub confirm_password: String
}

/* * custom validate email domain (gmail.com, icloud.com, yahoo.com, dan outlook.com) */
fn validate_email_domain(
    email: &str
) -> Result<(), ValidationError>
{
    let valid_domains = [
        "gmail.com",
        "icloud.com",
        "yahoo.com",
        "outlook.com",
    ];

    let email_parts = email.split('@').collect::<Vec<_>>();
    if email_parts.len() == 2 {
        let domain = email_parts[1];
        if valid_domains.iter().any(|&valid_domain| valid_domain == domain){
            Ok(())
        } else {
            Err( ValidationError::new("invalid email domain") )
        }
    } else {
        Err(
            ValidationError::new("invalid email format")
        )
    }
}
/* * end custom validate email domain (gmail.com, icloud.com, yahoo.com, dan outlook.com) */
/* * custom validate phone number (only receive digits) */
fn validate_phone_number(
    phone_number: &str
) -> Result<(), ValidationError> 
{
    if phone_number.chars().all(char::is_numeric) {
        Ok(())
    } else {
        Err(
            ValidationError::new("invalid digits")
        )
    }
}
/* * custom validate phone number (only receive digits) */
/* * custom validate password (upper, lower, digit, no whitespace, and min 8 characters) */
fn validate_password(
    password: &str
) -> Result<(), ValidationError>
{
    let mut has_whitespace = false;
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_digit = false;

    for character in password.chars() {
        has_whitespace |= character.is_whitespace();
        has_lowercase |= character.is_lowercase();
        has_uppercase |= character.is_uppercase();
        has_digit |= character.is_digit(10);
    }

    if !has_whitespace && has_lowercase && has_uppercase && has_digit && password.len() >= 8 {
        Ok(())
    } else {
        Err(
            ValidationError::new("password validation failed")
        )
    }
}
/* * custom validate password (upper, lower, digit, no whitespace, and min 8 characters) */


/* * custom deserializer function for convert string to lowercase  */
fn to_lowercase<'de, D>(
    deserializer: D
) -> Result<String, D::Error>
where D: serde::Deserializer<'de>
{
    let s: String = Deserialize::deserialize(deserializer)?;

    Ok(
        s.to_lowercase()
    )
}
/* * end custom deserializer function for convert string to lowercase  */
/* * custom deserialize function for convert Option<String> to lowercase */
fn to_option_lowercase<'de, D>(
    deserializer: D
) -> Result<Option<String>, D::Error>
where D: serde::Deserializer<'de>
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;

    match s {
        Some(text) => Ok(Some(text.to_lowercase())),
        None => Ok(None)
    }
}
/* * end custom deserialize function for convert Option<String> to lowercase */
/* * custom deserializer function for convert Option<String> to capitalize */
fn capitalize_option_words<'de, D>(
    deserializer: D
) -> Result<Option<String>, D::Error>
where D: serde::Deserializer<'de>
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;

    match s {
        Some(text) => {
            let lowercase_text = text.to_lowercase();
            let capitalized = lowercase_text
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");

            Ok(Some(capitalized))
        },
        None => Ok(None)
    }
}
/* * end custom deserializer function for convert Option<String> to capitalize */