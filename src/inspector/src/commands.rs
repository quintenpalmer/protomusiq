use std::path::PathBuf;

pub trait AppCmd {
    fn operate(&self, path: PathBuf);
}

pub trait FlexibleCmd {
    fn flex_operate(&self, args: Vec<String>);
}
