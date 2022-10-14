use lazy_static::lazy_static;
use regex::Regex;

fn get_continuation_key(text: &str) -> Option<regex::Match> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#""continuation":"(.*?)","#).unwrap();
    }
    let caps = RE.captures(text).unwrap();
    caps.get(1)
}

fn get_api_key(text: &str) -> Option<regex::Match> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#""INNERTUBE_API_KEY":"(.*?)","#).unwrap();
    }
    let caps = RE.captures(text).unwrap();
    caps.get(1)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = "OLKhm1jKrVM";
    let video_url = format!("https://www.youtube.com/watch?v={}", &video_id);

    let client = reqwest::Client::new();
    let body = client.get(&video_url).send().await?.text().await?;
    let continuation_key = get_continuation_key(&body).unwrap().as_str();
    let api_key = get_api_key(&body).unwrap().as_str();
    println!("{}, {}", continuation_key, api_key);
    Ok(())
}
