//! Errors created by this crate.
use crate::common::LinkDescription;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// A paypal api response error.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseError {
    /// The error name.
    pub name: String,
    /// The error message.
    pub message: String,
    /// Paypal debug id
    pub debug_id: String,
    /// Error details
    pub details: Vec<HashMap<String, String>>,
    /// Only available on Identity errors
    pub error: Option<String>,
    /// Only available on Identity errors
    pub error_description: Option<String>,
    /// Links with more information about the error.
    pub links: Vec<LinkDescription>,
}

impl fmt::Display for ApiResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for ApiResponseError {}

/// When a currency is invalid.
#[derive(Debug)]
pub struct InvalidCurrencyError(pub String);

impl fmt::Display for InvalidCurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{:?} is not a valid currency", self.0) 
    }
}

impl Error for InvalidCurrencyError {}
