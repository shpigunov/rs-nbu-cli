#[derive(PartialEq, Eq, Debug)]
pub enum Currency {
    Eur,
    Usd,
}

impl std::str::FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EUR" => Ok(Currency::Eur),
            "USD" => Ok(Currency::Usd),
            _ => Err(format!("Unknown currency: {}", s)),
        }
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Currency::Eur => write!(f, "EUR"),
            Currency::Usd => write!(f, "USD"),
        }
    }
}
