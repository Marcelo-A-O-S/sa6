#[derive(Debug)]
pub enum ErrorProject {
    NotFound(String),
    ErrorUpdateDB(String),
    ErrorGeneric(String)
}
pub enum ErrorServices{
    NotFound(String)
}