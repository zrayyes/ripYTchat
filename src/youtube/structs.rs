use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContiuationResponse {
    pub continuation_contents: ContinuationContents,
    pub tracking_params: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationContents {
    pub live_chat_continuation: LiveChatContinuation,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatContinuation {
    pub continuations: Vec<Continuation>,
    pub actions: Option<Vec<Action>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Continuation {
    pub live_chat_replay_continuation_data: Option<LiveChatReplayContinuationData>,
    pub player_seek_continuation_data: Option<PlayerSeekContinuationData>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatReplayContinuationData {
    pub time_until_last_message_msec: i64,
    pub continuation: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSeekContinuationData {
    pub continuation: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub replay_chat_item_action: ReplayChatItemAction,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayChatItemAction {
    pub actions: Vec<Action2>,
    pub video_offset_time_msec: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action2 {
    pub add_chat_item_action: Option<AddChatItemAction>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddChatItemAction {
    pub item: Item,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub live_chat_text_message_renderer: Option<LiveChatTextMessageRenderer>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTextMessageRenderer {
    pub message: Message,
    pub author_name: AuthorName,
    pub timestamp_text: TimestampText,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub runs: Vec<Run>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub text: Option<String>,
    pub emoji: Option<Emoji>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub emoji_id: String,
    pub shortcuts: Option<Vec<String>>,
    pub image: Image,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorName {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimestampText {
    pub simple_text: String,
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::Path;

    use super::*;
    use serde_json;

    #[test]
    fn test_serde_deserialize_works() -> Result<(), Box<dyn Error>> {
        let file_path = Path::new("./tests/data/post_body");
        let response: String = fs::read_to_string(file_path)?.parse()?;
        let deserialized: ContiuationResponse = serde_json::from_str(&response).unwrap();

        assert_eq!(
            deserialized.tracking_params,
            Some("CAAQ0b4BIhMI35XI_5Pl-gIV0fERCB2MTQd-".to_string())
        );
        assert_eq!(
            deserialized
                .continuation_contents
                .live_chat_continuation
                .actions
                .as_ref()
                .unwrap()
                .len(),
            101
        );

        let mut authors: Vec<String> = Vec::new();

        for action in deserialized
            .continuation_contents
            .live_chat_continuation
            .actions
            .unwrap()
        {
            for chat_item_action in action.replay_chat_item_action.actions {
                let author_name = match chat_item_action.add_chat_item_action {
                    Some(chat_action) => match chat_action.item.live_chat_text_message_renderer {
                        Some(message) => message.author_name.simple_text,
                        None => continue,
                    },
                    None => continue,
                };
                authors.push(author_name);
            }
        }

        assert_eq!(authors.len(), 100);
        Ok(())
    }
}
