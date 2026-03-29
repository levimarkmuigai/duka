use actix_files::NamedFile;
use actix_web::Result;
use std::path::PathBuf;

pub async fn dashboard_view() -> Result<NamedFile> {
    let path: PathBuf = "./frontend/admin/dashboard.html".parse()?;
    Ok(NamedFile::open(path)?)
}
