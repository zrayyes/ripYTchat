use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use crate::store::models::{Aggregate, Channel, Emote, Message, Video};
use crate::youtube::api::YoutubeApi;
use regex::Regex;

pub struct RequestHandler<Y>
where
    Y: YoutubeApi,
{
    pub youtube_api: Y,
}

impl<Y> RequestHandler<Y>
where
    Y: YoutubeApi,
{
    pub async fn get_video_info(
        &self,
        video_id: &str,
    ) -> Result<VideoInfo, Box<dyn std::error::Error>> {
        let body = self
            .youtube_api
            .get_video_page_body(video_id)
            .await
            .unwrap();
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

        Ok(VideoInfo {
            id: video_id.to_string(),
            api_key: api_key.to_string(),
            continuation_key: continuation_key.to_string(),
            title: title.to_string(),
            channel_name: channel_name.to_string(),
            channel_id: channel_id.to_string(),
        })
    }

    pub async fn get_all_chat_messages(
        &self,
        video_info: VideoInfo,
    ) -> Result<Aggregate, Box<dyn std::error::Error>> {
        let video = Video::new(video_info.id, video_info.title);
        let channel = Channel::new(video_info.channel_id, video_info.channel_name);
        let mut messages: Vec<Message> = vec![];
        let emotes: HashSet<Emote> = HashSet::new();

        let api_key = video_info.api_key;
        let mut continuation_key = Some(video_info.continuation_key);

        loop {
            let response = match continuation_key {
                Some(continuation_key) => {
                    self.youtube_api
                        .get_live_chat(&api_key, &continuation_key)
                        .await?
                }
                None => break,
            };
            // Update with new continuation key (if it exists)
            continuation_key = match response
                .continuation_contents
                .live_chat_continuation
                .continuations
                .first()
            {
                Some(continuation) => continuation
                    .live_chat_replay_continuation_data
                    .as_ref()
                    .map(|continuation_data| continuation_data.continuation.to_string()),
                None => None,
            };

            if continuation_key.is_none() {
                break;
            }

            for action in response
                .continuation_contents
                .live_chat_continuation
                .actions
                .unwrap()
            {
                let live_chat_message_renderer =
                    match &action.replay_chat_item_action.actions[0].add_chat_item_action {
                        Some(action) => match &action.item.live_chat_text_message_renderer {
                            Some(message) => message,
                            None => break,
                        },
                        None => break,
                    };
                let author = &live_chat_message_renderer.author_name.simple_text;
                let timestamp = action.replay_chat_item_action.video_offset_time_msec;
                let mut content = "".to_owned();
                for item in &live_chat_message_renderer.message.runs {
                    if item.text.is_some() {
                        content.push_str(item.text.as_ref().unwrap().as_str());
                    }
                    if item.emoji.is_some() {
                        content.push_str(match &item.emoji.as_ref().unwrap().shortcuts {
                            Some(shortcuts) => shortcuts[0].as_str(),
                            None => break,
                        })
                    }
                }
                let message = Message::new(content, author.clone(), timestamp);
                messages.push(message);
            }
            thread::sleep(Duration::from_millis(100));
            // TODO: Pull Emotes
        }

        let aggregate = Aggregate {
            video,
            channel,
            messages,
            emotes,
        };
        Ok(aggregate)
    }
}

#[derive(Debug)]
pub struct VideoInfo {
    id: String,
    api_key: String,
    continuation_key: String,
    title: String,
    channel_name: String,
    channel_id: String,
}

fn get_key_value_from_body<'a>(
    key: &'a str,
    text: &'a str,
) -> Result<Option<regex::Match<'a>>, String> {
    let re_str = format!(r#""{}":"(.*?)","#, key);
    let re = Regex::new(&re_str).unwrap_or_else(|_| panic!("Invalid regex: {}", &re_str));
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

    #[tokio::test]
    async fn test_get_video_info_valid() -> Result<(), Box<dyn std::error::Error>> {
        let fake_body = r#""continuation":"abc","INNERTUBE_API_KEY":"xyz","title":"MY_TITLE","author":"MY_CHANNEL","channelId":"0001","#;
        let mut mock_api = MockYoutubeApi::new();
        mock_api
            .expect_get_video_page_body()
            .with(eq("12345"))
            .times(1)
            .returning(|_| Ok(fake_body.to_string()));

        let handler = RequestHandler {
            youtube_api: mock_api,
        };
        let video_info = handler.get_video_info("12345").await?;
        assert_eq!("12345", video_info.id);
        assert_eq!("abc", video_info.continuation_key);
        assert_eq!("xyz", video_info.api_key);
        assert_eq!("MY_TITLE", video_info.title);
        assert_eq!("MY_CHANNEL", video_info.channel_name);
        assert_eq!("0001", video_info.channel_id);
        Ok(())
    }
}
