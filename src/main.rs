use std::{env, process::exit};

pub mod video;
pub mod youtube;

use video::{get_next_continuation, video_from_id};
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
    let ytc: YoutubeApiClient = YoutubeApiClient::init();
    let mut video = video_from_id(&ytc, video_id).await?;
    print!("{}", video);
    loop {
        let response = get_next_continuation(&video, &ytc).await?.unwrap();
        let continuation = match response
            .continuation_contents
            .live_chat_continuation
            .continuations
            .first()
        {
            Some(continuation) => match &continuation.live_chat_replay_continuation_data {
                Some(continuation_data) => Some(continuation_data.continuation.to_string()),
                None => None,
            },
            None => None,
        };
        println!(
            "{}",
            response
                .continuation_contents
                .live_chat_continuation
                .actions
                .unwrap_or(vec![])
                .len()
        );

        if continuation.is_none() {
            break;
        }
        // TODO: Move elsewhere
        video.set_continuation_key(continuation);
    }

    // TODO: Reorganize video struct, add manager
    //TODO: Write to file
    Ok(())
}
