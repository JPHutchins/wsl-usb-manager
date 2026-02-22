use std::io;
use std::path::{Path, PathBuf};

/// Temporary function that creates the folder and does an easy example.
pub fn ensure_settings_dir() -> Result<PathBuf, io::Error> {
    let path = std::env::var("LOCALAPPDATA")
        .map(|dir| PathBuf::from(dir).join("WSL USB Manager"))
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;

    std::fs::create_dir_all(&path)?;
    Ok(path)
}

/// Temporary example of saving some data.
pub fn write_persistent_example(dir: &Path) -> Result<(), io::Error> {
    use std::time::SystemTime;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default();
    std::fs::write(dir.join("persistent_example.txt"), timestamp)
}
