use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum CreateDatabaseError {
    #[error("Portfolio already exists")]
    DatabaseExists,
    #[error("Unable to determine home directory")]
    CantDetermineHomeDir,
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

pub(crate) fn new_db_path(
    path: Option<PathBuf>,
    name: &str,
) -> Result<PathBuf, CreateDatabaseError> {
    let base = path.unwrap_or(dirs::home_dir().ok_or(CreateDatabaseError::CantDetermineHomeDir)?);

    let path = base.join(".invest_folio").join(format!("{name}.db"));

    if path.exists() {
        return Err(CreateDatabaseError::DatabaseExists);
    }

    Ok(path)
}
pub(crate) async fn create_db(path: &Path) -> Result<String, CreateDatabaseError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let url = format!("sqlite://{}", path.to_string_lossy());

    if Sqlite::database_exists(&url).await? {
        return Err(CreateDatabaseError::DatabaseExists);
    }

    Sqlite::create_database(&url).await?;

    Ok(url)
}

#[cfg(test)]
mod test {
    use tempfile::tempdir;

    use super::*;
    #[test]
    fn home_db_path() {
        let path = new_db_path(None, "testing");
        let correct_path: PathBuf = dirs::home_dir()
            .unwrap()
            .join(".invest_folio")
            .join("testing.db");
        assert_eq!(path.unwrap(), correct_path)
    }
    #[test]
    fn custom_db_path() {
        let dir = tempdir().unwrap();

        let path = new_db_path(Some(dir.path().into()), "testing").unwrap();

        assert_eq!(path, dir.path().join(".invest_folio").join("testing.db"));
    }
    #[test]
    fn database_exists_error() {
        let dir = tempdir().unwrap();
        let db_dir = dir.path().join(".invest_folio");
        std::fs::create_dir_all(&db_dir).unwrap();
        let db_file = db_dir.join("testing.db");
        std::fs::File::create(&db_file).unwrap();

        let result = new_db_path(Some(dir.path().into()), "testing").unwrap_err();

        assert!(matches!(result, CreateDatabaseError::DatabaseExists));
    }
    #[tokio::test]
    async fn create_database_success() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("testing.db");
        let correct = format!("sqlite://{}", &path.display());
        let result = create_db(&path).await.unwrap();
        assert_eq!(result, correct);
    }
    #[tokio::test]
    async fn create_database_failure() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("testing.db");
        std::fs::File::create(&path).unwrap();
        let result = create_db(&path).await.unwrap_err();
        assert!(matches!(result, CreateDatabaseError::DatabaseExists))
    }
}
