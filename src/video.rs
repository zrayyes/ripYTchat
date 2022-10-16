use crate::youtube::{api::YoutubeApi, structs::ContiuationResponse};
use regex::Regex;

#[derive(Debug)]
pub struct Video {
    id: String,
    api_key: String,
    continuation_key: String,
    title: String,
    channel_name: String,
    channel_id: String,
}

impl Video {
    pub async fn from_id<Api: YoutubeApi>(
        youtube_api: &Api,
        video_id: &str,
    ) -> Result<Video, Box<dyn std::error::Error>> {
        let body = youtube_api.get_video_body(&video_id).await?;
        let continuation_key = get_key_value_from_body("continuation", &body)?
            .unwrap()
            .as_str();
        let api_key = get_key_value_from_body("INNERTUBE_API_KEY", &body)?
            .unwrap()
            .as_str();
        let title = get_key_value_from_body("title", &body)?.unwrap().as_str();
        let channel_name = get_key_value_from_body("author", &body)?.unwrap().as_str();
        let channel_id = get_key_value_from_body("channelId", &body)?
            .unwrap()
            .as_str();

        Ok(Video {
            id: video_id.to_string(),
            api_key: api_key.to_string(),
            continuation_key: continuation_key.to_string(),
            title: title.to_string(),
            channel_name: channel_name.to_string(),
            channel_id: channel_id.to_string(),
        })
    }

    pub async fn get_next_continuation<Api: YoutubeApi>(
        &self,
        youtube_api: &Api,
    ) -> Result<ContiuationResponse, Box<dyn std::error::Error>> {
        let out = youtube_api
            .get_live_chat_continuation(&self.api_key, &self.continuation_key)
            .await?;
        Ok(out)
    }
}

fn get_key_value_from_body<'a>(
    key: &'a str,
    text: &'a str,
) -> Result<Option<regex::Match<'a>>, String> {
    let re_str = format!(r#""{}":"(.*?)","#, key);
    let re = Regex::new(&re_str).expect(&format!("Invalid regex: {}", &re_str));
    let caps = match re.captures(text) {
        Some(caps) => caps,
        None => return Err(format!("Key '{}' not found in HTML body.", key)),
    };
    Ok(caps.get(1))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::youtube::api::MockYoutubeApi;
    use mockall::predicate::*;

    #[test]
    fn test_get_key_value_from_body_valid() -> Result<(), String> {
        let body = r#"{"continuation":"abc123",""#;
        let continuation_key = get_key_value_from_body("continuation", &body)?
            .unwrap()
            .as_str();

        Ok(assert_eq!(continuation_key, "abc123"))
    }

    #[test]
    fn test_get_key_value_from_body_key_missing() -> Result<(), String> {
        let key_name = "continuation";
        let body = r#"{"000":"abc123",""#;
        let error = get_key_value_from_body(key_name, &body).unwrap_err();

        Ok(assert_eq!(
            error,
            format!("Key '{}' not found in HTML body.", key_name)
        ))
    }

    #[tokio::test]
    async fn test_video_from_id_valid() -> Result<(), Box<dyn std::error::Error>> {
        let fake_body = r#""continuation":"abc","INNERTUBE_API_KEY":"xyz","title":"MY_TITLE","author":"MY_CHANNEL","channelId":"0001","#;
        let mut mock_api = MockYoutubeApi::new();
        mock_api
            .expect_get_video_body()
            .with(eq("12345"))
            .times(1)
            .returning(|_| Ok(fake_body.to_string()));

        let video = Video::from_id(&mock_api, "12345").await?;
        assert_eq!("12345", video.id);
        assert_eq!("abc", video.continuation_key);
        assert_eq!("xyz", video.api_key);
        assert_eq!("MY_TITLE", video.title);
        assert_eq!("MY_CHANNEL", video.channel_name);
        assert_eq!("0001", video.channel_id);
        Ok(())
    }
}
