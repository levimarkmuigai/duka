use actix_files::NamedFile;
use actix_web::Result;
use std::path::PathBuf;
use tracing::instrument;

#[instrument(name = "dashboard_view_handler")]
pub async fn dashboard_view() -> Result<NamedFile> {
    let path: PathBuf = "./frontend/admin/dashboard.html".parse()?;
    Ok(NamedFile::open(path)?)
}
