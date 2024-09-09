use std::path::PathBuf;

pub trait AppCmd {
    fn operate(&self, path: PathBuf);
}
