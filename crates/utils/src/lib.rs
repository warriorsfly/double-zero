#[macro_use]
extern crate lazy_static;

use actix_web::{error::BlockingError, http::{StatusCode, header}, HttpResponse, ResponseError, HttpResponseBuilder};
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use serde::{Deserialize, Serialize};
use std::{fmt, ops::Deref};

pub mod apub;
pub mod claims;
pub mod config;
pub mod constants;
pub mod encryption;
pub mod helpers;
pub mod middleware;
pub mod opration;
pub mod pool;
pub mod utils;
pub mod validate;
/// local user id
pub type UserId = usize;
/// websocket connection id
pub type ConnectionId = usize;
/// room id
pub type RoomId = usize;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct IpAddr(pub String);

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[macro_export]
macro_rules! location_info {
    () => {
        format!(
            "None value at {}:{}, column {}",
            file!(),
            line!(),
            column!()
        )
    };
}

#[derive(Debug, Display, PartialEq, Serialize)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    PaymentRequired(String),
    DataBaseError(String),
    Unauthorized(String),
    #[display(fmt = "")]
    ValidateError(Vec<String>),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse<T> {
    errors: Vec<T>,
}

impl<String> Deref for ErrorResponse<String> {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.errors
    }
}

/// custom error
impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::PaymentRequired(_) => StatusCode::PAYMENT_REQUIRED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
            .body(self.to_string())
    }
}

impl From<Vec<String>> for ErrorResponse<String> {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

/// Convert DBErrors to ServiceErrors
impl From<DBError> for Error {
    fn from(error: DBError) -> Error {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::BadRequest(message);
                }
                Error::DataBaseError("Unknown database error".into())
            }
            _ => Error::DataBaseError("Unknown database error".into()),
        }
    }
}

/// Convert PoolErrors to ServiceErrors
impl From<PoolError> for Error {
    fn from(error: PoolError) -> Error {
        Error::DataBaseError(error.to_string())
    }
}

/// Convert BlockingError to ServiceErrors
impl From<BlockingError> for Error {
    fn from(error: BlockingError) -> Error {
        Error::InternalServerError(error.to_string())
    }
}
