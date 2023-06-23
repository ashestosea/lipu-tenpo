use std::path::{Path, PathBuf};

use directories::{BaseDirs, ProjectDirs};

fn project_dirs() -> ProjectDirs {
    let _ = BaseDirs::new().expect("Can't read home directory!");

    let project_dir =
        ProjectDirs::from("", "ashestosea", "lipu-tenpo").expect("Can't read home directory!");

    if !project_dir.config_dir().exists() {
        std::fs::create_dir_all(project_dir.config_dir()).expect("Can't create config directory!");
    }

    if !project_dir.data_dir().exists() {
        std::fs::create_dir_all(project_dir.data_dir()).expect("Can't create data directory!");
    }

    project_dir
}

pub fn log_path() -> PathBuf {
    let dirs = project_dirs();
    let timelog = dirs.data_dir().join(Path::new("timelog.csv"));

    if !timelog.exists() {
        std::fs::write(&timelog, "").expect("Can't write timelog file!");
    }

    timelog
}
