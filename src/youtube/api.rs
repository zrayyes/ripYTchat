use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait YoutubeApi {
    fn init() -> Self;
    async fn get_video_body(&self, video_id: &str) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct YoutubeApiClient {
    client: reqwest::Client,
}

#[async_trait]
impl YoutubeApi for YoutubeApiClient {
    fn init() -> Self {
        let client = reqwest::Client::new();
        YoutubeApiClient { client: client }
    }

    async fn get_video_body(&self, video_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let video_url = format!("https://www.youtube.com/watch?v={}", &video_id);

        let body = &self
            .client
            .get(&video_url)
            .send()
            .await
            .expect("Failed to fetch video page from video ID.")
            .text()
            .await
            .expect("Failed to get video page text.");

        Ok(body.to_string())
    }
}
