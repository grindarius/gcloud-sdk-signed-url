use error::SignedURLError;
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
