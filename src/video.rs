use crate::youtube::YoutubeApi;
use regex::Regex;

#[derive(Debug)]
pub struct Video {
    id: String,
    api_key: String,
    first_continuation_key: String,
    title: String,
    channel_name: String,
    channel_id: String,
}

impl Video {
    pub async fn from_id<T: YoutubeApi>(
        youtube_api: T,
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
            first_continuation_key: continuation_key.to_string(),
            title: title.to_string(),
            channel_name: channel_name.to_string(),
            channel_id: channel_id.to_string(),
        })
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
}
