pub struct Video {
    id: String,
    title: String,
}
pub struct Message {
    content: String,
    author: String,
    offset: String,
}
pub struct Channel {
    id: String,
    name: String,
}
pub struct Emote {
    id: String,
    shortcut: String,
    url: String,
}

pub struct Aggregate {
    video: Video,
    channel: Channel,
    messages: Vec<Message>,
    emotes: Vec<Emote>,
}
