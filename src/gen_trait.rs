use errors::*;

pub trait CargoGenerator {
    fn gen(&self, short_name: &str, dry_run: bool) -> Result<()>;
}
