use std::{env, fs, path::Path};
use anyhow::{Context, Result};

fn main() -> anyhow::Result<()> {
    let path = get_path()?;
    let path = Path::new(&path);

    create_dir(&path)?;

    change_dir(&path)
}

fn change_dir(path: &Path) -> anyhow::Result<()> {
    env::set_current_dir(path).context(format!("Could not change directories to {}", path.display()))
}

fn get_path() -> Result<String> {
    env::args().next().context("Could not get argument")
}

fn create_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path).context(format!("Could not create directories {}", path.display()))
}
