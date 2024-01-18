#[derive(Debug)]
pub enum Error {
    ServerReportsDone,
}

pub fn run_client() -> Result<(), Error> {
    Err(Error::ServerReportsDone)
}
