use diesel::result::ConnectionError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("DATABASE_URL environment variable is not set")]
    NoDatabaseUrl,
    #[error("database connection failed: {0}")]
    ConnectionFailed(#[from] ConnectionError),
    #[error("failed to create connection pool: {0}")]
    ConnectionPoolFailed(#[from] r2d2::Error),
    #[error("database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
}
