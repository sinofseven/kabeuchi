use clap::crate_name;
use dirs::home_dir;
use std::path::PathBuf;

pub fn get_path_config_dir() -> Result<PathBuf, String> {
    home_dir()
        .map(|h| h.join(".config").join(crate_name!()))
        .ok_or_else(|| "failed to resolve Home directory".to_string())
}

pub fn get_path_config_file() -> Result<PathBuf, String> {
    get_path_config_dir().map(|h| h.join("config.toml"))
}

pub fn get_path_pre_messages_dir() -> Result<PathBuf, String> {
    get_path_config_dir().map(|p| p.join("pre_messages"))
}

pub fn get_path_profile_pre_messages_dir(profile_name: &str) -> Result<PathBuf, String> {
    get_path_pre_messages_dir().map(|p| p.join(profile_name))
}

pub fn get_path_history_dir() -> Result<PathBuf, String> {
    get_path_config_dir().map(|p| p.join("hiistory"))
}

pub fn get_path_profile_history_dir(profile_name: &str) -> Result<PathBuf, String> {
    get_path_history_dir().map(|p| p.join(profile_name))
}

pub fn get_path_cache_dir() -> Result<PathBuf, String> {
    get_path_config_dir().map(|p| p.join("cache"))
}

fn get_path_editting_message_dir() -> Result<PathBuf, String> {
    get_path_cache_dir().map(|p| p.join("editting_message"))
}

pub fn get_path_editting_message_file() -> Result<PathBuf, String> {
    get_path_editting_message_dir().map(|p| p.join("EDITTING_MESSAGE"))
}

pub fn get_files_in_dir(path: &PathBuf) -> Result<Vec<PathBuf>, String> {
    if !path.is_dir() {
        return Ok(Vec::new());
    }

    let mut result = Vec::new();

    let entries_all =
        std::fs::read_dir(path).map_err(|e| format!("failed to read directory: {e}"))?;

    for entry in entries_all {
        let entry =
            entry.map_err(|e| format!("failed to parse result of reading directory: {e}"))?;
        let path_file = entry.path();
        if !path_file.is_file() {
            continue;
        }
        result.push(path_file.clone());
    }
    result.sort();
    Ok(result)
}
