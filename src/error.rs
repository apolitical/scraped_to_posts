use diesel;
use std::{env, result, string::ToString};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Environment variable error: {:?}", _0)]
    EnvVar(env::VarError),
    #[fail(display = "Could not connect to DB: {:?}", _0)]
    DbConnection(diesel::ConnectionError),
    #[fail(display = "Could not get result from DB: {:?}", _0)]
    DbResultError(diesel::result::Error),
    #[fail(display = "Could not recover id from insertion into {}", table)]
    DbInsertError { table: String }
}

impl Error {
    pub fn db_insert_error<T>(table: T) -> Error
    where T: AsRef<str> + ToString
    {
        Error::DbInsertError { table: table.to_string() }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Error {
        Error::DbResultError(err)
    }
}
