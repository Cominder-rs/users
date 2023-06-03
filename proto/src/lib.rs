pub mod users_proto {
    tonic::include_proto!("auth");
}

pub use users_proto::*;

use leptos::{IntoView, View, Scope};
impl From<String> for CountryCode {
    fn from(country: String) -> Self {
        match country.as_str() {
            "AT" => CountryCode::At,
            "BY" => CountryCode::By,
            "BE" => CountryCode::Be,
            "CA" => CountryCode::Ca,
            "CN" => CountryCode::Cn,
            "DK" => CountryCode::Dk,
            "DE" => CountryCode::De,
            "FI" => CountryCode::Fi,
            "FR" => CountryCode::Fr,
            "GB" => CountryCode::Gb,
            "GE" => CountryCode::Ge,
            "IN" => CountryCode::In,
            "ID" => CountryCode::Id,
            "IT" => CountryCode::It,
            "JP" => CountryCode::Jp,
            "KZ" => CountryCode::Kz,
            "RU" => CountryCode::Ru,
            "TR" => CountryCode::Tr,
            "UA" => CountryCode::Ua,
            _ => CountryCode::Unknown
        }
    }
}

impl From<i32> for CountryCode {
    fn from(value: i32) -> Self {
        CountryCode::from_i32(value).expect("Converting i32 to CountryCode")
    }
}

impl IntoView for CountryCode {
    fn into_view(self, cx: Scope) -> View {
        (self as i32).into_view(cx)
    }
}

