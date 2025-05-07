use anyhow::{anyhow, Result};
use std::{env::current_dir, path::Path};

pub fn format_dir_pattern(path: String) -> Result<String> {
    let path = Path::new(&path);
    if !path.exists() {
        return Err(anyhow!("dir not exists"));
    }

    let pattern = if path.is_absolute() {
        path.join("**/*")
    } else {
        current_dir()?.join(path).join("**/*")
    };
    let pattern = pattern.to_string_lossy().into_owned();

    Ok(pattern)
}
