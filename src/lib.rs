use gcloud_sdk::google_rest_apis::storage_v1::Object;
use options::GetSignedUrlOptions;

pub mod options;

pub trait SignedUrlExt {
    fn get_signed_url(&self, _options: GetSignedUrlOptions) -> String;
}

impl SignedUrlExt for Object {
    fn get_signed_url(&self, options: GetSignedUrlOptions) -> String {}
}
