use std::fmt::Display;

use gcloud_sdk::{
    google::iam::credentials::v1::iam_credentials_client::IamCredentialsClient, GoogleApi,
    GoogleAuthMiddleware,
};
use options::SignedURLOptions;

mod buffer;
mod error;
mod method;
mod options;
mod scheme;
mod url_style;

pub async fn get_signed_url(
    bucket: &str,
    object: &str,
    options: SignedURLOptions,
    client: GoogleApi<IamCredentialsClient<GoogleAuthMiddleware>>,
) -> Result<(), SignedURLError> {
    todo!()
}

#[derive(Debug, Clone)]
pub enum SignedURLError {
    SometingWong,
}

impl Display for SignedURLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SometingWong => write!(f, "Someting wong guys"),
        }
    }
}

impl std::error::Error for SignedURLError {}
