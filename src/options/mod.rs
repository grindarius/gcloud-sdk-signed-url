use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use action::SignedUrlAction;
use http::HeaderMap;
use url_style::URLStyle;
use version::SignedUrlVersion;

pub mod action;
pub mod url_style;
pub mod version;

#[derive(Debug)]
pub struct GetSignedUrlOptions {
    host: String,
    signing_endpoint: String,
    action: SignedUrlAction,
    version: SignedUrlVersion,
    style: Option<URLStyle>,
    content_md5: Option<String>,
    content_type: Option<String>,
    expires: Duration,
    accessible_at: Option<SystemTime>,
    extension_headers: Option<HeaderMap>,
    prompt_save_as: Option<String>,
    response_disposition: Option<String>,
    response_type: Option<String>,
    query_params: Option<HashMap<String, Vec<String>>>,
}
