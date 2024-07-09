use std::env;

use entity::{
    db_enums::{CrateType, RepoSyncStatus},
    repo_sync_status,
};
use rdkafka::producer::FutureProducer;
use serde::{Deserialize, Serialize};

pub mod producer;

pub fn get_producer() -> FutureProducer {
    let brokers = env::var("KAFKA_BROKER").unwrap();
    producer::new_producer(&brokers)
}

#[derive(Serialize, Deserialize)]
pub struct RepoMessage {
    pub crate_name: String,
    pub github_url: Option<String>,
    pub mega_url: String,
    pub crate_type: CrateType,
    pub status: RepoSyncStatus,
    pub err_message: Option<String>,
}

impl From<repo_sync_status::ActiveModel> for RepoMessage {
    fn from(value: repo_sync_status::ActiveModel) -> Self {
        Self {
            crate_name: value.crate_name.unwrap(),
            github_url: value.github_url.unwrap(),
            mega_url: value.mega_url.unwrap(),
            crate_type: value.crate_type.unwrap(),
            status: value.status.unwrap(),
            err_message: value.err_message.unwrap(),
        }
    }
}
