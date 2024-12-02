use std::collections::BTreeMap;

use super::{
    options::SignedURLOptions,
    scheme::{self, SignedURLScheme},
};
use chrono::{DateTime, Utc};
use url::Url;

const CREDENTIALS_SCOPE: &str = "auto/storage/goog4_request";
const X_GOOG_ALGORITHM: &str = "GOOG4-RSA-SHA256";

pub fn get_signed_headers(headers: &[(String, String)], options: &SignedURLOptions) -> String {
    let mut header_names = headers.iter().map(|k| k.0.clone()).collect::<Vec<_>>();
    header_names.push("host".to_string());

    if options.content_type().is_some() {
        header_names.push("content-type".to_string());
    }

    if options.content_md5().is_some() {
        header_names.push("content-md5".to_string());
    }

    header_names.sort_unstable();
    header_names.join(";")
}

pub fn get_header_with_value(
    headers: &[(String, String)],
    host: String,
    options: &SignedURLOptions,
) -> Vec<String> {
    let mut header_with_value = vec![format!("host:{}", host)];
    header_with_value.extend_from_slice(
        &headers
            .iter()
            .map(|(h, v)| format!("{}:{}", h, v))
            .collect::<Vec<String>>(),
    );

    if let Some(ct) = options.content_type() {
        header_with_value.push(format!("content-type:{}", ct))
    }

    if let Some(md5) = options.content_md5() {
        header_with_value.push(format!("content-md5:{}", md5))
    }

    header_with_value.sort_unstable();
    header_with_value
}

pub async fn create_signed_buffer(
    bucket: String,
    object: String,
    options: SignedURLOptions,
) -> Vec<u8> {
    let start_time: DateTime<Utc> = options.start_time().into();

    let host = options.style().host(options.hostname().as_deref(), &bucket);
    let base_url = if options.insecure() {
        format!("http://{}", host)
    } else {
        format!("https://{}", host)
    };

    let mut signed_url = Url::parse(&base_url).unwrap();
    signed_url.set_path(&object);

    let headers = match options.scheme() {
        SignedURLScheme::V2 => scheme::v2::sanitize_headers(options.headers()),
        SignedURLScheme::V4 => scheme::v4::sanitize_headers(options.headers()),
    };
    let x_goog_signed_headers = get_signed_headers(&headers, &options);

    let x_goog_credential = format!(
        "{}/{}/{}",
        options.google_access_id(),
        start_time.format("%Y%m%d"),
        CREDENTIALS_SCOPE
    );
    let x_goog_date = start_time.format("%Y%m%dT%H%M%SZ").to_string();
    let x_goog_expires = options.expires().as_secs().to_string();

    let mut query_params = BTreeMap::from_iter(options.query_parameters());
    let _ = query_params.insert("X-Goog-Credential".to_string(), vec![x_goog_credential]);
    let _ = query_params.insert(
        "X-Goog-SignedHeaders".to_string(),
        vec![x_goog_signed_headers],
    );
    let _ = query_params.insert("X-Goog-Date".to_string(), vec![x_goog_date]);
    let _ = query_params.insert("X-Goog-Expires".to_string(), vec![x_goog_expires]);
    let _ = query_params.insert(
        "X-Goog-Algorithm".to_string(),
        vec![X_GOOG_ALGORITHM.to_string()],
    );

    {
        let mut query = signed_url.query_pairs_mut();
        for (k, values) in &query_params {
            for value in values {
                query.append_pair(k, value);
            }
        }
    }

    let header_with_value = get_header_with_value(&headers, host, &options);
    let signed_headers = get_signed_headers(&headers, &options);

    // INFO: We can unwrap because we just added the value in
    let escaped_query = signed_url.query().unwrap().replace('+', "%20");

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(format!("{}\n", options.method()).into_bytes());
    buffer.extend(format!("{}\n", options.content_md5()).into_bytes());
    buffer.extend(format!("{}\n", options.content_type()).into_bytes());
    buffer.extend(format!("{}\n", options.expires()));

    let _ = format!(
        "{}\n{}\n{}\n{}\n\n{}",
        options.method(),
        signed_url.path().replace('+', "%20"),
        escaped_query,
        header_with_value.join("\n"),
        signed_headers
    )
    .into_bytes();

    todo!()
}
