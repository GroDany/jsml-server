use crate::database::Database;
use crate::database::{self, DatabaseError};
use crate::source;

pub struct State {
    database: Database,
    // source: Source,
}

impl State {
    pub fn new(path: &str, id_key: &str) -> Result<Self, DatabaseError> {
        Err(DatabaseError::from("test"))
    }
}
