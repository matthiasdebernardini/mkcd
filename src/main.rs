use anyhow::{Context, Result};
use std::{env, fs, path::Path};

fn main() -> Result<()> {
    let mut cwd = env::current_dir()?;
    dbg!(cwd.clone());
    dbg!(env::args().collect::<Vec<String>>());
    let new_dir = env::args().collect::<Vec<String>>();

    let path = Path::new(&new_dir[1]);
    dbg!(path.clone());

    cwd.extend(path);

    dbg!(cwd.clone());
    fs::create_dir_all(path).context(format!("Could not create directories {}", path.display()))?;
    env::set_current_dir(path).context(format!(
        "Could not change directories to {}",
        path.display()
    ))

}
