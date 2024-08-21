mod extractor;
mod matcher;

pub const URL: &str = "https://github.com/2571-project/postgresql-binaries";

pub use extractor::extract;
pub use matcher::matcher;
