use std::fs::OpenOptions;

use chrono::prelude::*;

pub struct OpenFile {
    pub tree_id: u32,
    pub share_name: String,
    pub path: String,
    pub fd: u32,
    pub open_options: OpenOptions,
    pub opened_at: DateTime<Utc>,
}
