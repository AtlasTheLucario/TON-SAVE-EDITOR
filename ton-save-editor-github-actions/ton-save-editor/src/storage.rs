use anyhow::{Context, Result};
use chrono::Local;
use std::path::{Path, PathBuf};

fn backup_dir() -> Result<PathBuf> {
    let base = dirs::data_local_dir().context("Could not locate the Windows local application-data directory")?;
    let dir = base.join("ToNSaveEditor").join("Backups");
    std::fs::create_dir_all(&dir).with_context(|| format!("Could not create {}", dir.display()))?;
    Ok(dir)
}

pub fn create_backup(contents: &str) -> Result<PathBuf> {
    let dir = backup_dir()?;
    let stamp = Local::now().format("%Y%m%d-%H%M%S-%3f");
    let path = dir.join(format!("editor-buffer-{stamp}.txt"));
    std::fs::write(&path, contents).with_context(|| format!("Could not write backup {}", path.display()))?;
    Ok(path)
}

pub fn backup_file(source: &Path) -> Result<PathBuf> {
    let dir = backup_dir()?;
    let stamp = Local::now().format("%Y%m%d-%H%M%S-%3f");
    let name = source.file_name().and_then(|s| s.to_str()).unwrap_or("save");
    let path = dir.join(format!("{name}.{stamp}.bak"));
    std::fs::copy(source, &path)
        .with_context(|| format!("Could not back up {} to {}", source.display(), path.display()))?;
    Ok(path)
}
