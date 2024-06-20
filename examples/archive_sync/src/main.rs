#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use postgresql_archive::blocking::{extract, get_archive};
use postgresql_archive::{Result, DEFAULT_RELEASES_URL, LATEST};

fn main() -> Result<()> {
    let (archive_version, archive) = get_archive(DEFAULT_RELEASES_URL, &LATEST)?;
    let out_dir = tempfile::tempdir()?.into_path();
    extract(&archive, &out_dir)?;
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
