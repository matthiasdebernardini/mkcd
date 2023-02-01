use anyhow::{Context, Result};
use std::{env, fs, path::Path};

fn main() -> Result<()> {
    let mut cwd = env::current_dir()?;

    let path = env::args().nth(1).context("Could not get argument for directory name")?;

    let path = Path::new(&path);

    cwd.extend(path);

    fs::create_dir_all(&cwd)
        .context(format!("Could not create directories {}", path.display()))?;

    env::set_current_dir(&cwd).context(format!(
        "Could not change directories to {}",
        path.display()
    ))
}
