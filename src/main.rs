use anyhow::{Context, Result};
use std::{env, fs, path::Path};

fn main() -> Result<()> {
    let mut cwd = env::current_dir()?;

    let new_dir = env::args().collect::<Vec<String>>();

    let path = Path::new(&new_dir[1]);

    cwd.extend(path);

    fs::create_dir_all(cwd.clone())
        .context(format!("Could not create directories {}", path.display()))?;

    env::set_current_dir(cwd.clone()).context(format!(
        "Could not change directories to {}",
        path.display()
    ))
}
