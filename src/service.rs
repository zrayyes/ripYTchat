use crate::handler::RequestHandler;
use crate::store::Store;
use crate::youtube::api::YoutubeApi;

pub struct Service<S, Y>
where
    S: Store,
    Y: YoutubeApi,
{
    pub store: S,
    pub handler: RequestHandler<Y>,
}

impl<S, Y> Service<S, Y>
where
    S: Store,
    Y: YoutubeApi,
{
    pub async fn store_chat_messages(
        &self,
        video_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let video_info = &self.handler.get_video_info(&video_id).await?;
        print!("{:?}", video_info);
        Ok(())
    }
}
