use super::currency;
use reqwest::blocking::Client;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Response {
    r030: u32,
    txt: String,
    rate: f32,
    cc: String,
    exchangedate: String,
}

pub fn get_rate(currency: currency::Currency) -> f32 {
    Client::new()
        .get("https://bank.gov.ua/NBUStatService/v1/statdirectory/exchange")
        .query(&[("json", ""), ("valcode", &currency.to_string())])
        .send()
        .expect("Failed to send request")
        .json::<Vec<Response>>()
        .expect("Failed to parse response")
        .first()
        .expect("Failed to get first element")
        .rate
}
