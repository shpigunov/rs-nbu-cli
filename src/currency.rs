use strum::Display;
use strum_macros::EnumString;

#[derive(PartialEq, Eq, Debug, EnumString, Display, Clone)]
pub enum Currency {
    #[strum(serialize = "EUR", serialize = "eur")]
    Eur,
    #[strum(serialize = "USD", serialize = "usd")]
    Usd,
    #[strum(serialize = "GBP", serialize = "gbp")]
    Gbp,
    #[strum(serialize = "JPY", serialize = "jpy")]
    Jpy,
}
