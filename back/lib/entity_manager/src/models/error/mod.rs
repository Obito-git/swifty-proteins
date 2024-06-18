pub enum DatabaseError {
    UniqueViolation(String),
    InternalError,
}
