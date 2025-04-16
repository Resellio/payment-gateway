use actix_web::http::StatusCode;

use crate::common::models::{AppResult, ErrorResponse};

use super::models::ProcessPaymentRequest;

#[derive(Debug, PartialEq)]
pub enum Currency {
    Pln,
    Usd,
}

#[derive(Debug)]
pub struct Payment {
    pub(super) amount: f64,
    pub(super) currency: Currency,
    pub(super) card_number: String,
    pub(super) card_expiry_month: u8,
    pub(super) card_expiry_year: u16,
    pub(super) cvv: [char; 3],
}

pub fn process_payment(payment: &Payment) -> AppResult<String> {
    todo!();
    Ok("transaction_id".into())
}

impl TryFrom<ProcessPaymentRequest> for Payment {
    type Error = ErrorResponse;

    fn try_from(value: ProcessPaymentRequest) -> Result<Self, Self::Error> {
        if value.force_error {
            return Err(ErrorResponse::new(
                "Forced error".into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        let currency = match value.currency.as_str() {
            "PLN" => Currency::Pln,
            "USD" => Currency::Usd,
            _ => {
                return Err(ErrorResponse::new(
                    "Invalid currency".into(),
                    StatusCode::BAD_REQUEST,
                ));
            }
        };
        if value.card_number.len() < 13 || value.card_number.len() > 19 {
            return Err(ErrorResponse::new(
                "Invalid card number length".into(),
                StatusCode::BAD_REQUEST,
            ));
        }
        let (card_expiry_month_str, card_expiry_year_str) =
            value.card_expiry.split_once("/").ok_or(ErrorResponse::new(
                "Invalid card expiry date format".into(),
                StatusCode::BAD_REQUEST,
            ))?;
        let card_expiry_month = card_expiry_month_str.parse::<u8>().map_err(|_| {
            ErrorResponse::new("Invalid card expiry month".into(), StatusCode::BAD_REQUEST)
        })?;
        let card_expiry_year = card_expiry_year_str.parse::<u16>().map_err(|_| {
            ErrorResponse::new("Invalid card expiry year".into(), StatusCode::BAD_REQUEST)
        })?;
        if value.cvv.len() != 3 {
            return Err(ErrorResponse::new(
                "Invalid cvv length".into(),
                StatusCode::BAD_REQUEST,
            ));
        }
        if value.cvv.chars().any(|c| !c.is_ascii_digit()) {
            return Err(ErrorResponse::new(
                "Invalid cvv".into(),
                StatusCode::BAD_REQUEST,
            ));
        }
        let mut cvv_chars = value.cvv.chars();
        // Note: we can safely unwrap here as we have already checked the lenght of the cvv.
        let cvv = [
            cvv_chars.next().unwrap(),
            cvv_chars.next().unwrap(),
            cvv_chars.next().unwrap(),
        ];

        Ok(Payment {
            amount: value.amount,
            currency,
            card_number: value.card_number,
            card_expiry_month,
            card_expiry_year,
            cvv,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;

    fn valid_request() -> ProcessPaymentRequest {
        ProcessPaymentRequest {
            amount: 100.0,
            currency: "PLN".to_string(),
            card_number: "1234567890123".to_string(),
            card_expiry: "12/2025".to_string(),
            cvv: "123".to_string(),
            force_error: false,
        }
    }

    #[test]
    fn test_payment_try_from_ppr_valid_conversion() {
        let request = valid_request();
        let result = Payment::try_from(request);
        assert!(result.is_ok());
        let payment = result.unwrap();
        assert_eq!(payment.amount, 100.0);
        assert_eq!(payment.currency, Currency::Pln);
        assert_eq!(payment.card_expiry_month, 12);
        assert_eq!(payment.card_expiry_year, 2025);
        assert_eq!(payment.cvv, ['1', '2', '3']);
    }

    #[test]
    fn test_payment_try_from_ppr_forced_error() {
        let mut request = valid_request();
        request.force_error = true;
        let result = Payment::try_from(request);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error, "Forced error");
        assert_eq!(err.code, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_payment_try_from_ppr_invalid_currency() {
        let mut request = valid_request();
        request.currency = "EUR".to_string();
        let result = Payment::try_from(request);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.error, "Invalid currency");
        assert_eq!(err.code, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_payment_try_from_ppr_invalid_card_number_length() {
        let mut request = valid_request();
        request.card_number = "123".to_string();
        let result = Payment::try_from(request);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error, "Invalid card number length");

        let mut request = valid_request();
        request.card_number = "1".repeat(20); // too long
        let result = Payment::try_from(request);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error, "Invalid card number length");
    }

    #[test]
    fn test_payment_try_from_ppr_invalid_card_expiry_format() {
        let mut request = valid_request();
        request.card_expiry = "122025".to_string(); // missing slash
        let result = Payment::try_from(request);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error, "Invalid card expiry date format");
    }

    #[test]
    fn test_payment_try_from_ppr_invalid_card_expiry_month() {
        let mut request = valid_request();
        request.card_expiry = "xx/2025".to_string(); // non-numeric month
        let result = Payment::try_from(request);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error, "Invalid card expiry month");
    }

    #[test]
    fn test_payment_try_from_ppr_invalid_card_expiry_year() {
        let mut request = valid_request();
        request.card_expiry = "12/xxxx".to_string(); // non-numeric year
        let result = Payment::try_from(request);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error, "Invalid card expiry year");
    }

    #[test]
    fn test_payment_try_from_ppr_invalid_cvv_length() {
        let mut request = valid_request();
        request.cvv = "12".to_string(); // too short
        let result = Payment::try_from(request);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error, "Invalid cvv length");
    }

    #[test]
    fn test_payment_try_from_ppr_invalid_cvv_characters() {
        let mut request = valid_request();
        request.cvv = "12x".to_string(); // non-digit
        let result = Payment::try_from(request);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error, "Invalid cvv");
    }
}
