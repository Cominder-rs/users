use derive_more::Display;

#[derive(Display)]
pub enum AuthError {
    #[display(fmt = "InvalidPhoneNumber")]
    InvalidPhoneNumber,
    #[display(fmt = "InvalidConfirmationCode")]
    InvalidConfirmationCode,
    #[display(fmt = "LoginSessionNotFound")]
    LoginSessionNotFound,
    #[display(fmt = "InvalidUsername")]
    InvalidUsername,
    #[display(fmt = "InvalidFirstname")]
    InvalidFirstname,
    #[display(fmt = "InvalidLastname")]
    InvalidLastname,
    #[display(fmt = "InvalidCity")]
    InvalidCity,
    #[display(fmt = "UsernameBusy")]
    UsernameBusy,
    Unknown
}

impl From<&str> for AuthError {
    fn from(value: &str) -> Self {
        match value {
            "InvalidPhoneNumber" => Self::InvalidPhoneNumber,
            "InvalidConfirmationCode" => Self::InvalidConfirmationCode,
            "LoginSessionNotFound" => Self::LoginSessionNotFound,
            "InvalidUsername" => Self::InvalidUsername,
            "InvalidFirstname" => Self::InvalidFirstname,
            "InvalidLastanme" => Self::InvalidLastname,
            "InvalidCity" => Self::InvalidCity,
            _ => Self::Unknown,
        }
    }
}