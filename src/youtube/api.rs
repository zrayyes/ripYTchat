use super::structs::ContiuationResponse;
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait YoutubeApi {
    fn init() -> Self;
    async fn get_video_page_body(
        &self,
        video_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_live_chat_continuation(
        &self,
        api_key: &str,
        continuation: &str,
    ) -> Result<ContiuationResponse, Box<dyn std::error::Error>>;
}

pub struct YoutubeApiClient {
    client: reqwest::Client,
}

#[async_trait]
impl YoutubeApi for YoutubeApiClient {
    fn init() -> Self {
        let client = reqwest::Client::new();
        YoutubeApiClient { client }
    }

    async fn get_video_page_body(
        &self,
        video_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let video_url = format!("https://www.youtube.com/watch?v={}", &video_id);

        let body = &self
            .client
            .get(&video_url)
            .send()
            .await
            .expect("Failed to send get_video_page_body request.")
            .text()
            .await
            .expect("Failed to extract text from get_video_page_body.");

        Ok(body.to_string())
    }

    async fn get_live_chat_continuation(
        &self,
        api_key: &str,
        continuation: &str,
    ) -> Result<ContiuationResponse, Box<dyn std::error::Error>> {
        let video_chat_replay_url = format!(
            "https://www.youtube.com/youtubei/v1/live_chat/get_live_chat_replay?key={}",
            &api_key
        );

        let res = &self
            .client
            .post(video_chat_replay_url)
            .body(format!(r#"{{"context":{{"client":{{"clientName":"WEB","clientVersion":"2.20210909.07.00"}}}},"continuation":"{}"}}"#, &continuation))
            .send()
            .await.expect("Failed to send get_live_chat_replay request.").text().await.expect("Failed to extract text from get_live_chat_replay.");

        let deserialized: ContiuationResponse = serde_json::from_str(res)
            .expect("Failed to deserialize JSON response from get_live_chat_replay.");
        Ok(deserialized)
    }
}
