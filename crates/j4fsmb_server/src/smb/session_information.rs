use chrono::prelude::*;

use super::transport_connection::TransportConnection;

pub struct SessionInformation {
    pub connection: TransportConnection,
    pub session_id: u64,
    pub session_key: String,
    pub created_at: DateTime<Utc>,
}