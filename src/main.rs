use regex::Regex;

#[derive(Debug)]
struct Video {
    id: String,
    api_key: String,
    first_continuation_key: String,
    title: String,
    channel_name: String,
    channel_id: String,
}

impl Video {
    async fn from_id(video_id: &str) -> Video {
        let video_url = format!("https://www.youtube.com/watch?v={}", &video_id);

        let body = reqwest::get(&video_url)
            .await
            .expect("Failed to fetch video page from video ID.")
            .text()
            .await
            .expect("Failed to get video page text.");
        let continuation_key = get_key_value_from_body("continuation", &body)
            .unwrap()
            .as_str();
        let api_key = get_key_value_from_body("INNERTUBE_API_KEY", &body)
            .unwrap()
            .as_str();
        let title = get_key_value_from_body("title", &body).unwrap().as_str();
        let channel_name = get_key_value_from_body("author", &body).unwrap().as_str();
        let channel_id = get_key_value_from_body("channelId", &body)
            .unwrap()
            .as_str();

        Video {
            id: video_id.to_string(),
            api_key: api_key.to_string(),
            first_continuation_key: continuation_key.to_string(),
            title: title.to_string(),
            channel_name: channel_name.to_string(),
            channel_id: channel_id.to_string(),
        }
    }
}

fn get_key_value_from_body<'a>(key: &'a str, text: &'a str) -> Option<regex::Match<'a>> {
    let re_str = format!(r#""{}":"(.*?)","#, key);
    let re = Regex::new(&re_str).unwrap();
    let caps = re.captures(text).unwrap();
    caps.get(1)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Read video id from args
    let video_id = "OLKhm1jKrVM";
    let video = Video::from_id(video_id).await;
    print!("{:?}", video);
    //TODO: get_live_chat_replay
    //TODO: parse body
    //TODO: Write to file
    Ok(())
}
