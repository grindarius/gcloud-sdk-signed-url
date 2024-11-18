#[derive(Debug)]
pub enum SignedUrlAction {
    Read,
    Write,
    Delete,
    Resumable,
}
