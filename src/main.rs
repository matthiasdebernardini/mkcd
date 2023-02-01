use anyhow::{Context, Result};
use std::{env, fs, path::Path};

fn main() -> anyhow::Result<()> {
    let path = env::args().next().context("Could not get argument")?;
    let path = Path::new(&path);

    env::set_current_dir(path).context(format!(
        "Could not change directories to {}",
        path.display()
    ))?;

    fs::create_dir_all(path).context(format!("Could not create directories {}", path.display()))
}
