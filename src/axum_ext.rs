use std::fmt::Display;
use axum::http::StatusCode;
use axum::Json;

pub type SimpleResponse<T> = Result<T, (StatusCode, String)>;

pub trait ResponseWrapResult<T>: Sized {
    fn wrap(self, status: StatusCode) -> SimpleResponse<T>;

    fn wrap_server_error(self) -> SimpleResponse<T> {
        self.wrap(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn wrap_client_error(self) -> SimpleResponse<T> {
        self.wrap(StatusCode::BAD_REQUEST)
    }
}

impl<T, U> ResponseWrapResult<T> for Result<T, U> where U: Display {
    fn wrap(self, status: StatusCode) -> SimpleResponse<T> {
        self.map_err(|x| (status, x.to_string()))
    }
}

pub trait ResponseWrapOption<T>: Sized {
    fn wrap_error(self, status: StatusCode, message: impl Into<String>) -> SimpleResponse<T>;
    fn wrap_server_error(self) -> SimpleResponse<T> {
        self.wrap_error(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    }

    fn wrap_client_error(self) -> SimpleResponse<T> {
        self.wrap_error(StatusCode::BAD_REQUEST, "Bad Request")
    }
}

impl<T> ResponseWrapOption<T> for Option<T> {
    fn wrap_error(self, status: StatusCode, message: impl Into<String>) -> SimpleResponse<T> {
        self.ok_or((status, message.into()))
    }
}

pub type ObjectResponse<T> = SimpleResponse<Json<T>>;
pub type BinaryResponse = SimpleResponse<Vec<u8>>;