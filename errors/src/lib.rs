use from_str_macro::FromStr;

#[derive(Eq, PartialEq, FromStr, Clone, Copy)]
pub enum AuthError {
    InvalidPhoneNumber,
    InvalidConfirmationCode,
    LoginSessionNotFound,
    InvalidUsername,
    InvalidFirstname,
    InvalidLastname,
    InvalidCity,
    PendingRegistryNotFound,
    UsernameBusy,
    Unknown
}

