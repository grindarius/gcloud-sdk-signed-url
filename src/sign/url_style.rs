use std::fmt::Debug;

const DEFAULT_HOST: &str = "storage.googleapis.com";

#[derive(Debug, Default, Clone)]
pub enum URLStyle {
    #[default]
    Path,
    VirtualHosted,
    BucketBoundHostname {
        hostname: String,
    },
}

impl URLStyle {
    pub fn host(&self, hostname: Option<&str>, bucket: &str) -> String {
        match self {
            Self::Path => {
                if let Some(custom_host) = hostname {
                    if !custom_host.is_empty() {
                        return strip_scheme(custom_host).to_string();
                    }
                }

                DEFAULT_HOST.to_string()
            }
            Self::VirtualHosted => {
                if let Some(custom_host) = hostname {
                    if !custom_host.is_empty() {
                        return format!("{}.{}", bucket, strip_scheme(custom_host));
                    }
                }

                format!("{}.{}", bucket, DEFAULT_HOST)
            }
            Self::BucketBoundHostname { hostname } => hostname.to_string(),
        }
    }

    pub fn path(&self, bucket: &str, object: &str) -> String {
        match self {
            Self::Path => {
                if object.is_empty() {
                    return bucket.to_string();
                }
                format!("{bucket}/{object}")
            }
            Self::VirtualHosted => object.to_string(),
            Self::BucketBoundHostname { hostname: _ } => object.to_string(),
        }
    }
}

fn strip_scheme(host: &str) -> &str {
    if host.contains("://") {
        let mut chunks = host.splitn(2, "://");
        let _protocol = chunks.next();
        return chunks.next().unwrap_or(host);
    }

    host
}
