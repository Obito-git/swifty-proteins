use diesel::result::Error as DieselDatabaseError;

pub enum DatabaseError {
    UniqueViolation(String),
    InternalError,
}

impl From<DieselDatabaseError> for DatabaseError {
    fn from(e: DieselDatabaseError) -> Self {
        match e {
            DieselDatabaseError::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                info,
            ) => DatabaseError::UniqueViolation(info.message().to_string()),
            _ => DatabaseError::InternalError,
        }
    }
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::UniqueViolation(message) => write!(f, "Unique violation: {}", message),
            DatabaseError::InternalError => write!(f, "Internal error"),
        }
    }
}
