use errors::*;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub struct FileHelper {
    dry_run: bool,
}

impl FileHelper {
    pub fn new(dry_run: bool) -> FileHelper {
        FileHelper { dry_run }
    }

    pub fn create_file<P>(&self, path: P, content: &str) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let the_path = path.as_ref();
        if let Some(dir) = the_path.parent() {
            create_dir_all(dir)?;
        }
        if self.dry_run {
            if the_path.exists() {
                Err(format!("{} already exists", the_path.display()).into())
            } else {
                Ok(())
            }
        } else {
            File::create(&path)
                .chain_err(|| format!("{} could not be created", the_path.display()))?
                .write_all(content.as_bytes())
                .chain_err(|| format!("Writing to {} failed", the_path.display()))
        }
    }

    pub fn modify_file<P, F>(&self, path: P, modifier: F) -> Result<()>
    where
        F: FnOnce(String) -> Result<Option<String>>,
        P: AsRef<Path>,
    {
        let mut content = String::new();
        File::open(&path)?.read_to_string(&mut content)?;
        if let Some(modified_content) = modifier(content)? {
            let mut writable_file = OpenOptions::new()
                .write(true)
                .open(&path)
                .chain_err(|| format!("{} could not be written", path.as_ref().display()))?;
            if self.dry_run {
                Ok(())
            } else {
                let content_bytes = modified_content.as_bytes();
                writable_file.set_len(content_bytes.len() as u64)?;
                writable_file
                    .write_all(content_bytes)
                    .chain_err(|| format!("Writing to {} failed", path.as_ref().display()))
            }
        } else {
            Ok(())
        }
    }
}
