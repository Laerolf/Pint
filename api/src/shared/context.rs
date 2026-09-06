use std::sync::Arc;

use sea_orm::ConnectionTrait;

/// Represents the context of the API.
#[derive(Clone)]
pub struct ApiContext<C: ConnectionTrait> {
    db_connection: Arc<C>,
}

impl<C: ConnectionTrait> ApiContext<C> {
    /// Creates a new [`Context`].
    pub fn new(db_connection: Arc<C>) -> Self {
        Self { db_connection }
    }

    pub fn db_connection(&self) -> &C {
        &self.db_connection
    }
}
