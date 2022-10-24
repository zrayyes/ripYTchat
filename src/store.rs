use std::fs::File;
use std::io::prelude::*;

use async_trait::async_trait;
pub mod models;

use models::Aggregate;

#[async_trait]
pub trait Store {
    async fn store_messages(
        &self,
        video_aggregate: Aggregate,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct SQLStore {}

#[async_trait]
impl Store for SQLStore {
    async fn store_messages(
        &self,
        video_aggregate: Aggregate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

pub struct FileStore {}

#[async_trait]
impl Store for FileStore {
    async fn store_messages(
        &self,
        video_aggregate: Aggregate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_name = format!("{}.txt", video_aggregate.video);
        let mut file = File::create(file_name)?;
        write!(file, "{}", video_aggregate.video)?;
        Ok(())
    }
}
