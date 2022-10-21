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
    pub async fn run(&self, video_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let video_info = self.handler.get_video_info(&video_id).await?;
        let video_messages = self.handler.get_all_chat_messages(video_info).await?;
        println!("{}", video_messages.messages.len());
        Ok(())
    }
}
