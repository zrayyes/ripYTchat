use std::{env, process::exit};

pub mod video;

use video::Video;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please add video id.");
        println!("Example: ripytc dPX0_IEXVRo");
        exit(1)
    }

    let video_id = &args[1];
    let video = Video::from_id(video_id).await;
    print!("{:?}", video);
    //TODO: get_live_chat_replay
    //TODO: parse body
    //TODO: Write to file
    Ok(())
}
