use std::{env, process::exit};

pub mod handler;
pub mod service;
pub mod store;
pub mod youtube;

use handler::RequestHandler;
use service::Service;
use store::SQLStore;
use youtube::api::{YoutubeApi, YoutubeApiClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please add video id.");
        println!("Example: ripytc dPX0_IEXVRo");
        exit(1)
    }

    let video_id = &args[1];
    let store: SQLStore = SQLStore {};
    let youtube_api: YoutubeApiClient = YoutubeApiClient::init();
    let handler = RequestHandler { youtube_api };
    let service = Service { store, handler };
    service.run(video_id).await?;

    Ok(())
}
