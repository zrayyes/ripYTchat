use async_trait::async_trait;
pub mod models;

use models::Video;

#[async_trait]
pub trait Store {
    async fn store_messages_for_video(&self, _video: Video) {
        todo!()
    }
}

pub struct SQLStore {}

#[async_trait]
impl Store for SQLStore {
    async fn store_messages_for_video(&self, _video: Video) {
        todo!()
    }
}
