use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait YoutubeApi {
    async fn get_video_body(&self, video_id: &str) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct YoutubeApiClient {}

#[async_trait]
impl YoutubeApi for YoutubeApiClient {
    async fn get_video_body(&self, video_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let video_url = format!("https://www.youtube.com/watch?v={}", &video_id);

        let response = reqwest::get(&video_url)
            .await
            .expect("Failed to fetch video page from video ID.");

        let body = response
            .text()
            .await
            .expect("Failed to get video page text.");

        Ok(body)
    }
}
