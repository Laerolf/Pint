use std::sync::Arc;

use sea_orm::ConnectionTrait;

/// Represents the context of the API.
#[derive(Clone)]
pub struct Context<C: ConnectionTrait> {
    db_connection: Arc<C>,
}

impl<C: ConnectionTrait> Context<C> {
    /// Creates a new [`Context`].
    pub fn new(db_connection: Arc<C>) -> Self {
        Self { db_connection }
    }

    pub fn db_connection(&self) -> &C {
        &self.db_connection
    }
}
