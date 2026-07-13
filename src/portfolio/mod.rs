use std::path::PathBuf;

use crate::portfolio::create::{create_db, new_db_path};

mod create;
pub struct Porfolio {
    name: String,
    db_url: String,
}

impl Porfolio {
    pub async fn new(path: Option<PathBuf>, name: String) -> anyhow::Result<Porfolio> {
        println!("Creating portfolio: {}...", &name);

        let path = new_db_path(path, &name)?;
        let db_url = create_db(&path).await?;

        Ok(Self { name, db_url })
    }
}
