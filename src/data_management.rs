use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use directories::ProjectDirs;

pub fn get_dir() -> Result<PathBuf, Error> {
    let proj = ProjectDirs::from("com", "fhist", "fhist");

    let data_path = if let Some(project) = proj {
        project.data_dir().to_path_buf()
    } else {
        return Err(Error::new(
            ErrorKind::NotFound,
            "project directory not available",
        ));
    };

    Ok(data_path)
}

pub fn ensure_dir() -> Result<(), Error> {
    let data_path = get_dir()?;
    fs::create_dir_all(&data_path)?;
    Ok(())
}
