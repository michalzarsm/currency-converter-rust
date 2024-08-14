use reqwest::get;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use anyhow::Result;
use thiserror::Error;
use crate::config::read_config;

// The API for the exchange rate service is provided by https://v6.exchangerate-api.com.
// I know reading the API key every time is not optimal, but i think it's good enough for this project.

const BASE_API_URL: &str = "https://v6.exchangerate-api.com/v6";

#[derive(Error, Debug)]
enum RequestError {
    #[error("Unsupported currency.")]
    UnsupportedCurrency,
    #[error("Malformed request.")]
    MalformedRequest,
    #[error("Invalid API key.")]
    InvalidApiKey,
    #[error("Inactive account.")]
    InactiveAccount,
    #[error("Quota reached.")]
    QuotaReached,
    #[error("Unknown error.")]
    UnknownError
}

#[derive(Serialize, Deserialize)]
pub struct ApiMultirateResponse {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub conversion_rates: Value
}

#[derive(Serialize, Deserialize)]
pub struct ApiRateResponse {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub target_code: String,
    pub conversion_rate: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ApiConversionResponse {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub target_code: String,
    pub conversion_rate: f64,
    pub conversion_result: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub result: String,
    #[serde(rename = "error-type")]
    pub error_type: String,
}

pub async fn get_all_exchange_rates(base_currency: &str) -> Result<ApiMultirateResponse> {
    let api_key = read_config()?.api_key;
    let url = format!("{}/{}/latest/{}", BASE_API_URL, api_key, base_currency);
    let response = get(&url).await?;
    let status = response.status();
    if !status.is_success() {
        let error = match response.json::<ErrorResponse>().await {
            Ok(error) => error,
            Err(_) => return Err(RequestError::UnknownError.into())
        };
        match error.error_type.as_str() {
            "unsupported-code" => return Err(RequestError::UnsupportedCurrency.into()),
            "malformed-request" => return Err(RequestError::MalformedRequest.into()),
            "invalid-key" => return Err(RequestError::InvalidApiKey.into()),
            "inactive-account" => return Err(RequestError::InactiveAccount.into()),
            "quota-reached" => return Err(RequestError::QuotaReached.into()),
            _ => return Err(RequestError::UnknownError.into())
        }
    }
    let exchange_rate_response = response.json::<ApiMultirateResponse>().await?;
    Ok(exchange_rate_response)
}

pub async fn get_exchange_rate(from: &str, to: &str) -> Result<ApiRateResponse> {
    let api_key = read_config()?.api_key;
    let url = format!("{}/{}/pair/{}/{}", BASE_API_URL, api_key, from, to);
    let response = get(&url).await?;
    let status = response.status();
    if !status.is_success() {
        let error = match response.json::<ErrorResponse>().await {
            Ok(error) => error,
            Err(_) => return Err(RequestError::UnknownError.into())
        };
        match error.error_type.as_str() {
            "unsupported-code" => return Err(RequestError::UnsupportedCurrency.into()),
            "malformed-request" => return Err(RequestError::MalformedRequest.into()),
            "invalid-key" => return Err(RequestError::InvalidApiKey.into()),
            "inactive-account" => return Err(RequestError::InactiveAccount.into()),
            "quota-reached" => return Err(RequestError::QuotaReached.into()),
            _ => return Err(RequestError::UnknownError.into())
        }
    }
    let exchange_rate_response = response.json::<ApiRateResponse>().await?;
    Ok(exchange_rate_response)
}

pub async fn convert(from: &str, to: &str, amount: f64) -> Result<ApiConversionResponse> {
    let api_key = read_config()?.api_key;
    let url = format!("{}/{}/pair/{}/{}/{}", BASE_API_URL, api_key, from, to, amount);
    let response = get(&url).await?;
    let status = response.status();
    if !status.is_success() {
        let error = match response.json::<ErrorResponse>().await {
            Ok(error) => error,
            Err(_) => return Err(RequestError::UnknownError.into())
        };
        match error.error_type.as_str() {
            "unsupported-code" => return Err(RequestError::UnsupportedCurrency.into()),
            "malformed-request" => return Err(RequestError::MalformedRequest.into()),
            "invalid-key" => return Err(RequestError::InvalidApiKey.into()),
            "inactive-account" => return Err(RequestError::InactiveAccount.into()),
            "quota-reached" => return Err(RequestError::QuotaReached.into()),
            _ => return Err(RequestError::UnknownError.into())
        }
    }
    let exchange_rate_response = response.json::<ApiConversionResponse>().await?;
    Ok(exchange_rate_response)
}

#[cfg(test)]
mod tests {
    use super::{get_all_exchange_rates, get_exchange_rate, convert};

    #[tokio::test]
    async fn test_get_all_exchange_rates_correct() {
        let response = get_all_exchange_rates("USD").await;
        match response {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
            }
            Err(e) => {
                panic!("Error getting exchange rates: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_all_exchange_rates_incorrect() {
        match get_all_exchange_rates("UST").await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_get_rate_both_correct() {
        match get_exchange_rate("USD", "EUR").await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_rate.is_nan(), false);
            }
            Err(e) => {
                panic!("Error getting exchange rate: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_rate_left_wrong() {
        match get_exchange_rate("UST", "EUR").await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_get_rate_right_wrong() {
        match get_exchange_rate("USD", "EUX").await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_get_rate_both_wrong() {
        match get_exchange_rate("UST", "EUX").await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_convert_all_correct() {
        match convert("USD", "EUR", 100.into()).await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_result.is_nan(), false);
            }
            Err(e) => {
                panic!("Error converting currency: {}", e);
            }
        }

        match convert("USD", "EUR", 100.0.into()).await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_result.is_nan(), false);
            }
            Err(e) => {
                panic!("Error converting currency: {}", e);
            }
        }

        match convert("USD", "EUR", 4231.1296.into()).await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_result.is_nan(), false);
            }
            Err(e) => {
                panic!("Error converting currency: {}", e);
            }
        }
    }

    
    #[tokio::test]
    async fn test_convert_wrong_left_currency() {
        match convert("UST", "EUR", 100.into()).await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_convert_wrong_right_currency() {
        match convert("USD", "EUX", 100.into()).await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_convert_small_amount() {
        match convert("USD", "EUR", 0.00025.into()).await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_result.is_nan(), false);
            }
            Err(e) => {
                panic!("Error converting currency: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_convert_big_amount() {
        match convert("USD", "EUR", 326235234543.32452362323.into()).await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_result.is_nan(), false);
            }
            Err(e) => {
                panic!("Error converting currency: {}", e);
            }
        }
    }
}

