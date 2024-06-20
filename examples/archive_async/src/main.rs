#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use postgresql_archive::{extract, get_archive, Result, DEFAULT_RELEASES_URL, LATEST};

#[tokio::main]
async fn main() -> Result<()> {
    let (archive_version, archive) = get_archive(DEFAULT_RELEASES_URL, &LATEST).await?;
    let out_dir = tempfile::tempdir()?.into_path();
    extract(&archive, &out_dir).await?;
    println!(
        "PostgreSQL {} extracted to {}",
        archive_version,
        out_dir.to_string_lossy()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
