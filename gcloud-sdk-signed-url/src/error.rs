use base64::DecodeError;
use gcloud_sdk::tonic::Status;

#[derive(Debug, thiserror::Error)]
pub enum SignedURLOptionsBuilderError {
    #[error("Invalid expiration date, expiration duration cannot be zero.")]
    ExpiresZero,
    #[error(
        "Invalid expiration date, expiration date cannot be longer than 7 days with v4 sign url."
    )]
    V4ExpiresTooLong,
    #[error("Google access id cannot be empty.")]
    EmptyGoogleAccessId,
    #[error("base64 decode error: {0}")]
    Base64DecodeError(#[from] DecodeError),
    #[error("Invalid md5 checksum provided.")]
    InvalidChecksum,
    #[error("Invalid url style for V2, only `PathStyle` is valid for V2 signing options.")]
    V2InvalidURLStyle,
    #[error("Invalid expiration duration, expiration duration cannot be greater than 7 days.")]
    InvalidExpirationDuration,
}

#[derive(Debug, thiserror::Error)]
pub enum SignedURLError {
    #[error("Builder error: {0}")]
    BuilderError(#[from] SignedURLOptionsBuilderError),
    #[error("Tonic error: {0}")]
    TonicError(#[from] Status),
}
