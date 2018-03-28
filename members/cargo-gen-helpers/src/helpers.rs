use errors::*;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn create_file<P>(path: P, content: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let the_path = path.as_ref();
    if let Some(dir) = the_path.parent() {
        create_dir_all(dir)?;
    }
    File::create(&path)
        .chain_err(|| format!("{} could not be created", the_path.display()))?
        .write_all(content.as_bytes())
        .chain_err(|| format!("Writing to {} failed", the_path.display()))
}

pub fn modify_file<P, F>(path: P, modifier: F) -> Result<()>
where
    F: FnOnce(String) -> Result<Option<String>>,
    P: AsRef<Path>,
{
    let mut content = String::new();
    File::open(&path)?.read_to_string(&mut content)?;
    if let Some(modified_content) = modifier(content)? {
        OpenOptions::new()
            .write(true)
            .open(&path)
            .chain_err(|| format!("{} could not be written", path.as_ref().display()))?
            .write_all(modified_content.as_bytes())
            .chain_err(|| format!("Writing to {} failed", path.as_ref().display()))
    } else {
        Ok(())
    }
}
