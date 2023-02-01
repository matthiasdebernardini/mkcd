use anyhow::{Context, Result};
use std::{env, fs, path::Path};

fn main() -> Result<()> {
    let mut cwd = env::current_dir()?;
    let new_dir = env::args().next().context("Could not get argument")?;
    let path = Path::new(&new_dir);

    cwd.extend(path);

    fs::create_dir_all(path).context(format!("Could not create directories {}", path.display()))?;
    env::set_current_dir(path).context(format!(
        "Could not change directories to {}",
        path.display()
    ))

}
