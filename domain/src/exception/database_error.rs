use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum DatabaseError {
    Unexpected(Box<dyn Error + Send + Sync>),
}

impl DatabaseError {

    pub fn get_message(&self) -> String {
        match self {
            DatabaseError::Unexpected(err) => {
                format!("Unexpected database error: {}", err)
            }
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DatabaseError::Unexpected(err) => Some(err.as_ref())
        }
    }
}

unsafe impl Send for DatabaseError {}
unsafe impl Sync for DatabaseError {}