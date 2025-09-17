use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an database error occurred: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("an error in email address occurred: {0}")]
    EmailAddressError(#[from] lettre::address::AddressError),
    #[error("an error in email content occurred: {0}")]
    EmailContentError(#[from] lettre::error::Error),
    #[error("an smtp error occurred: {0}")]
    SmtpError(#[from] lettre::transport::smtp::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::DatabaseError(_)
            | Self::EmailAddressError(_)
            | Self::EmailContentError(_)
            | Self::SmtpError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
