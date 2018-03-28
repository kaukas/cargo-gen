use errors::*;

pub trait CargoGenerator {
    fn gen(&self) -> Result<()>;
}
