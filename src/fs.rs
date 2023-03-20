use std::path::PathBuf;

pub fn load_text(path: &PathBuf) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| {
        format!(
            "failed to read text file: path={}, err={}",
            path.display(),
            e
        )
    })
}

pub fn save_text(path: &PathBuf, text: &str) -> Result<(), String> {
    let dir = path.parent().ok_or(format!(
        "failed to resolve parenet directory of file: path={}",
        path.display()
    ))?;
    std::fs::create_dir_all(dir).map_err(|e| {
        format!(
            "failed to create parent directory of path: path={}, err={}",
            path.display(),
            e
        )
    })?;
    std::fs::write(path, text)
        .map_err(|e| format!("failed to write file: path={}, err={}", path.display(), e))
}
