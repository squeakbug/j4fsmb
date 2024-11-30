use chrono::prelude::*;

use super::open_file::OpenFile;

pub struct Session {
    pub client_endpoint: String,
    pub user_name: String,
    pub machine_name: String,
    pub open_files: Vec<OpenFile>,
    pub created_at: DateTime<Utc>,
}