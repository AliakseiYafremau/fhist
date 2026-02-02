use std::{fs, io::Error, path::PathBuf};

use directories::ProjectDirs;

fn get_dir() -> Option<PathBuf> {
    let proj = ProjectDirs::from("com", "fhist", "fhist");

    let data_path = if let Some(project) = proj {
        project.data_dir().to_path_buf()
    } else {
        return None;
    };

    Some(data_path)
}

pub fn ensure_dir() -> Result<(), Error> {
    if let Some(data_path) = get_dir() {
        fs::create_dir_all(&data_path)?;
    }
    Ok(())
}
