use strum::Display;
use strum_macros::EnumString;

#[derive(PartialEq, Eq, Debug, EnumString, Display)]
pub enum Currency {
    #[strum(serialize = "EUR")]
    Eur,
    #[strum(serialize = "USD")]
    Usd,
}
