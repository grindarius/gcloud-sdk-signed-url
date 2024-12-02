use std::fmt::Display;

use gcloud_sdk::{
    google::iam::credentials::v1::iam_credentials_client::IamCredentialsClient, GoogleApi,
    GoogleAuthMiddleware,
};
use sign::options::SignedURLOptions;

mod sign;

pub async fn get_signed_url(
    bucket: String,
    object: String,
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
