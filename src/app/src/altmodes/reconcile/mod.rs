#[derive(Debug)]
pub enum Error {}

pub fn entry_point() -> Result<(), Error> {
    println!("let's reconcile");
    Ok(())
}
