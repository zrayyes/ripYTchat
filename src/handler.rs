use std::collections::HashSet;

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
        let messages: Vec<Message> = vec![];
        let emotes: HashSet<Emote> = HashSet::new();
        let mut continuation_key = video_info.continuation_key;

        //TODO: Parse

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
