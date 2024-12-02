use std::{collections::HashMap, time::SystemTime};

use chrono::{DateTime, Utc};
use url::Url;

use crate::sign::{options::SignedURLOptions, url_style::URLStyle};

const X_GOOG_ENCRYPTION_KEY_HEADER: &str = "x-goog-encryption-key";
const X_GOOG_ENCRYPTION_KEY_SHA_256_HEADER: &str = "x-goog-encryption-key-sha-256";

pub fn sanitize_headers(headers: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut sanitized_headers: Vec<(String, String)> = Vec::new();

    for (h, v) in headers {
        let sanitized_header = h.trim().to_lowercase();
        let sanitized_value = v.trim();

        // Exclude non canonical headers.
        if !sanitized_header.starts_with("x-goog") {
            continue;
        }

        // Exclude these headers.
        if sanitized_header == X_GOOG_ENCRYPTION_KEY_HEADER
            || sanitized_header == X_GOOG_ENCRYPTION_KEY_SHA_256_HEADER
        {
            continue;
        }

        sanitized_headers.push((sanitized_header, sanitized_value.to_string()));
    }

    sanitized_headers.sort_unstable_by(|a, b| {
        let a = format!("{}:{}", a.0, a.1);
        let b = format!("{}:{}", b.0, b.1);

        a.cmp(&b)
    });
    sanitized_headers
}

pub async fn signed_url(bucket: String, object: String, options: SignedURLOptions) -> String {
    let sanitized_headers = sanitize_headers(options.headers());
    // Strict path style because v2 only support this style.
    let host = URLStyle::Path.host(options.hostname().as_deref(), &bucket);

    let expiration_unix: DateTime<Utc> =
        (options.start_time().unwrap_or(SystemTime::now()) + options.expires()).into();

    let mut buffer: Vec<u8> = format!(
        "{}\n{}\n{}\n{}\n",
        options.method(),
        options.content_md5().unwrap_or("".to_string()),
        options.content_type().unwrap_or("".to_string()),
        expiration_unix.timestamp()
    )
    .into_bytes();

    if !sanitized_headers.is_empty() {
        for (h, v) in sanitized_headers {
            buffer.extend(&format!("{}:{}\n", h, v).into_bytes());
        }
    }

    let mut signed_url = Url::parse(&format!("https://{}/{}/{}", host, bucket, object)).unwrap();

    todo!()
}
