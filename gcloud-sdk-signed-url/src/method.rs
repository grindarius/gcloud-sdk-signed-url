use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SignedURLMethod {
    #[default]
    Get,
    Delete,
    Head,
    Put,
    Post,
}

impl Display for SignedURLMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Get => "GET",
                Self::Delete => "DELETE",
                Self::Head => "HEAD",
                Self::Put => "PUT",
                Self::Post => "POST",
            }
        )
    }
}
