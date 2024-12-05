use std::{
    fmt::Debug,
    time::{Duration, SystemTime},
};

use base64::{prelude::BASE64_STANDARD, Engine};

use crate::error::SignedURLOptionsBuilderError;

use super::{method::SignedURLMethod, scheme::SignedURLScheme, url_style::URLStyle};

const ONE_WEEK: Duration = Duration::from_secs(604800);
const MD5_HASH_SIZE: usize = 16;

#[derive(Debug)]
pub struct SignedURLOptions {
    google_access_id: String,
    start_time: Option<SystemTime>,
    method: SignedURLMethod,
    expires: Duration,
    content_type: Option<String>,
    headers: Vec<(String, String)>,
    query_parameters: Vec<(String, String)>,
    content_md5: Option<String>,
    style: URLStyle,
    insecure: bool,
    scheme: SignedURLScheme,
    hostname: Option<String>,
}

impl SignedURLOptions {
    pub fn new(google_access_id: String, expires: Duration) -> SignedURLOptionsBuilder {
        SignedURLOptionsBuilder {
            google_access_id,
            start_time: None,
            method: SignedURLMethod::Get,
            expires,
            content_type: None,
            headers: None,
            query_parameters: None,
            content_md5: None,
            style: URLStyle::default(),
            insecure: None,
            scheme: SignedURLScheme::default(),
            hostname: None,
        }
    }

    pub fn start_time(&self) -> Option<&SystemTime> {
        self.start_time.as_ref()
    }

    pub fn insecure(&self) -> bool {
        self.insecure
    }

    pub fn style(&self) -> &URLStyle {
        &self.style
    }

    pub fn hostname(&self) -> Option<&str> {
        self.hostname.as_deref()
    }

    pub fn scheme(&self) -> SignedURLScheme {
        self.scheme
    }

    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    pub fn content_md5(&self) -> Option<&str> {
        self.content_md5.as_deref()
    }

    pub fn google_access_id(&self) -> &str {
        &self.google_access_id
    }

    pub fn expires(&self) -> Duration {
        self.expires
    }

    pub fn query_parameters(&self) -> &[(String, String)] {
        &self.query_parameters
    }

    pub fn method(&self) -> SignedURLMethod {
        self.method
    }
}

#[derive(Debug)]
pub struct SignedURLOptionsBuilder {
    google_access_id: String,
    start_time: Option<SystemTime>,
    method: SignedURLMethod,
    expires: Duration,
    content_type: Option<String>,
    headers: Option<Vec<(String, String)>>,
    query_parameters: Option<Vec<(String, String)>>,
    content_md5: Option<String>,
    style: URLStyle,
    insecure: Option<bool>,
    scheme: SignedURLScheme,
    hostname: Option<String>,
}

impl SignedURLOptionsBuilder {
    pub fn start_time(&mut self, start_time: SystemTime) -> &mut Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn method(&mut self, method: SignedURLMethod) -> &mut Self {
        self.method = method;
        self
    }

    pub fn content_type(&mut self, content_type: String) -> &mut Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn headers(&mut self, headers: Vec<(String, String)>) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn query_parameters(&mut self, query_parameters: Vec<(String, String)>) -> &mut Self {
        self.query_parameters = Some(query_parameters);
        self
    }

    pub fn content_md5(&mut self, content_md5: String) -> &mut Self {
        self.content_md5 = Some(content_md5);
        self
    }

    pub fn style(&mut self, style: URLStyle) -> &mut Self {
        self.style = style;
        self
    }

    pub fn insecure(&mut self, insecure: bool) -> &mut Self {
        self.insecure = Some(insecure);
        self
    }

    pub fn scheme(&mut self, scheme: SignedURLScheme) -> &mut Self {
        self.scheme = scheme;
        self
    }

    pub fn hostname(&mut self, hostname: String) -> &mut Self {
        self.hostname = Some(hostname);
        self
    }

    pub fn build(self) -> Result<SignedURLOptions, SignedURLOptionsBuilderError> {
        if self.expires.is_zero() {
            return Err(SignedURLOptionsBuilderError::ExpiresZero);
        }

        if self.google_access_id.is_empty() {
            return Err(SignedURLOptionsBuilderError::EmptyGoogleAccessId);
        }

        if let Some(md5) = &self.content_md5 {
            let decoded = BASE64_STANDARD
                .decode(md5)
                .map_err(SignedURLOptionsBuilderError::Base64DecodeError)?;

            if decoded.len() != MD5_HASH_SIZE {
                return Err(SignedURLOptionsBuilderError::InvalidChecksum);
            }
        }

        if matches!(self.scheme, SignedURLScheme::V2) && !matches!(self.style, URLStyle::Path) {
            return Err(SignedURLOptionsBuilderError::V2InvalidURLStyle);
        }

        if self.expires > ONE_WEEK {
            return Err(SignedURLOptionsBuilderError::InvalidExpirationDuration);
        }

        Ok(SignedURLOptions {
            google_access_id: self.google_access_id,
            start_time: self.start_time,
            method: self.method,
            expires: self.expires,
            content_type: self.content_type,
            headers: self.headers.unwrap_or_default(),
            query_parameters: self.query_parameters.unwrap_or_default(),
            content_md5: self.content_md5,
            style: self.style,
            insecure: self.insecure.unwrap_or(false),
            scheme: self.scheme,
            hostname: self.hostname,
        })
    }
}
