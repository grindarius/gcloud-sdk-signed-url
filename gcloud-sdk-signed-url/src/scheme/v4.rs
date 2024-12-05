use std::{collections::BTreeMap, sync::LazyLock};

use gcloud_sdk::{
    google::iam::credentials::v1::iam_credentials_client::IamCredentialsClient, GoogleApi,
    GoogleAuthMiddleware,
};
use regex::Regex;
use url::Url;

use crate::{error::SignedURLError, options::SignedURLOptions};

static SPACE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(" +").unwrap());
static TAB_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("[\\t]+").unwrap());

pub fn sanitize_headers(headers: &[(String, String)]) -> Vec<(String, String)> {
    let mut sanitized_headers: BTreeMap<String, String> = BTreeMap::new();

    for (h, v) in headers {
        let sanitized_header = h.trim().to_lowercase();
        let trimmed_value = v.trim();
        let space_concatenated_value = SPACE_REGEX.replace_all(trimmed_value, " ");
        let sanitized_value = TAB_REGEX.replace_all(&space_concatenated_value, "\t");

        let _ = sanitized_headers.insert(sanitized_header, sanitized_value.to_string());
    }

    let mut sanitized_headers_string: Vec<(String, String)> = Vec::new();
    for (h, v) in sanitized_headers {
        sanitized_headers_string.push((h, v));
    }

    sanitized_headers_string
}

pub fn signed_url(
    bucket: &str,
    object: &str,
    options: SignedURLOptions,
    client: &GoogleApi<IamCredentialsClient<GoogleAuthMiddleware>>,
) -> Result<Url, SignedURLError> {
    let sanitized_headers = sanitize_headers(options.headers());

    let signed_url = Url::parse(&format!(
        "{}://{}/{}",
        if options.insecure() { "https" } else { "http" },
        options.style().host(options.hostname(), &bucket),
        &options.style().path(&bucket, &object),
    ))
    .unwrap();

    let mut buffer: String = format!("{}\n{}\n", options.method(), signed_url.path());

    todo!()
}
