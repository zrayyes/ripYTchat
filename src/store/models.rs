use core::fmt;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Video {
    id: String,
    title: String,
}

#[derive(Debug)]
pub struct Message {
    content: String,
    author: String,
    timestamp: String,
}

#[derive(Debug)]
pub struct Channel {
    id: String,
    name: String,
}

#[derive(Debug)]
pub struct Emote {
    id: String,
    shortcut: String,
    url: String,
}

pub struct Aggregate {
    pub video: Video,
    pub channel: Channel,
    pub messages: Vec<Message>,
    pub emotes: HashSet<Emote>,
}

impl Video {
    pub fn new(id: String, title: String) -> Self {
        Video { id, title }
    }
}

impl fmt::Display for Video {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.id, self.title)
    }
}

impl Message {
    pub fn new(content: String, author: String, timestamp: String) -> Self {
        Message {
            content,
            author,
            timestamp,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}---{}---{}", self.timestamp, self.author, self.content)
    }
}

impl Channel {
    pub fn new(id: String, name: String) -> Self {
        Channel { id, name }
    }
}

impl Emote {
    pub fn new(id: String, shortcut: String, url: String) -> Self {
        Emote { id, shortcut, url }
    }
}

impl PartialEq for Emote {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
