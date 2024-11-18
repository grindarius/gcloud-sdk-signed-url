#[derive(Debug)]
pub enum URLStyle {
    Path,
    BucketBoundHostname { domain: String },
    VirtualHost,
}

impl URLStyle {
    /// Default host of the URL.
    const HOST: &str = "storage.googleapis.com";

    pub fn host(&self, bucket: &str) -> String {
        match self {
            Self::Path => Self::HOST.to_string(),
            Self::VirtualHost => format!("{}.{}", bucket, Self::HOST),
            Self::BucketBoundHostname { domain } => domain.to_string(),
        }
    }

    pub fn path(&self, bucket: &str, object: &str) -> String {
        match self {
            Self::Path => {
                if object.is_empty() {
                    return bucket.to_string();
                }

                format!("{}/{}", bucket, object)
            }
            Self::VirtualHost => object.to_string(),
            Self::BucketBoundHostname { domain: _ } => object.to_string(),
        }
    }
}
